use std::{
    cmp::Ordering,
    collections::HashSet,
    fs::File,
    io::{self, Read, Write}
};

use alloy_primitives::BlockNumber;
use angstrom_types::primitive::PeerId;

const ROUND_ROBIN_CACHE: &str = "./";

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct AngstromValidator {
    peer_id:      PeerId,
    voting_power: u64,
    priority:     f64
}

impl AngstromValidator {
    pub fn new(name: PeerId, voting_power: u64) -> Self {
        AngstromValidator { peer_id: name, voting_power, priority: 0.0 }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WeightedRoundRobin {
    validators:                HashSet<AngstromValidator>,
    new_joiner_penalty_factor: f64,
    block_number:              BlockNumber
}

impl WeightedRoundRobin {
    pub fn new(
        validators: Vec<AngstromValidator>,
        block_number: BlockNumber,
        new_joiner_penalty_factor: Option<f64>
    ) -> Self {
        let file_path = format!("{}/state.json", ROUND_ROBIN_CACHE);
        if let Ok(mut file) = File::open(file_path) {
            let mut contents = String::new();
            if file.read_to_string(&mut contents).is_ok() {
                if let Ok(state) = serde_json::from_str(&contents) {
                    return state;
                }
            }
        }
        WeightedRoundRobin {
            validators: HashSet::from_iter(validators),
            new_joiner_penalty_factor: new_joiner_penalty_factor.unwrap_or(1.125),
            block_number
        }
    }

    fn proposer_selection(&mut self) -> PeerId {
        let total_voting_power: u64 = self.validators.iter().map(|v| v.voting_power).sum();

        let mut updated_validators = HashSet::new();
        for mut validator in self.validators.drain() {
            validator.priority += validator.voting_power as f64;
            updated_validators.insert(validator);
        }
        self.validators = updated_validators;

        let mut proposer = self
            .validators
            .iter()
            .max_by(|a, b| {
                a.priority
                    .partial_cmp(&b.priority)
                    .unwrap_or(Ordering::Equal)
            })
            .unwrap()
            .clone();
        proposer.priority -= total_voting_power as f64;
        let proposer_name = proposer.peer_id;
        self.validators.replace(proposer);

        proposer_name
    }

    fn center_priorities(&mut self) {
        let avg_priority: f64 =
            self.validators.iter().map(|v| v.priority).sum::<f64>() / self.validators.len() as f64;
        let mut updated_validators = HashSet::new();
        for mut validator in self.validators.drain() {
            validator.priority -= avg_priority;
            updated_validators.insert(validator);
        }
        self.validators = updated_validators;
    }

    fn scale_priorities(&mut self) {
        let max_priority = self
            .validators
            .iter()
            .map(|v| v.priority)
            .fold(f64::NEG_INFINITY, f64::max);
        let min_priority = self
            .validators
            .iter()
            .map(|v| v.priority)
            .fold(f64::INFINITY, f64::min);
        let total_voting_power: u64 = self.validators.iter().map(|v| v.voting_power).sum();
        let diff = max_priority - min_priority;
        let threshold = 2.0 * total_voting_power as f64;

        if diff > threshold {
            let scale = diff / threshold;
            let mut updated_validators = HashSet::new();
            for mut validator in self.validators.drain() {
                validator.priority /= scale;
                updated_validators.insert(validator);
            }
            self.validators = updated_validators;
        }
    }

    pub fn choose_proposer(&mut self, block_number: BlockNumber) -> Option<PeerId> {
        let rounds_to_catchup = (block_number - self.block_number) as usize;
        let mut leader = None;
        for _ in 0..rounds_to_catchup {
            self.center_priorities();
            self.scale_priorities();
            leader = Some(self.proposer_selection());
        }
        self.block_number = block_number;
        leader
    }

    fn remove_validator(&mut self, peer_id: &PeerId) {
        let validator = AngstromValidator::new(*peer_id, 0);
        self.validators.remove(&validator);
    }

    fn add_validator(&mut self, peer_id: PeerId, voting_power: u64) {
        let mut new_validator = AngstromValidator::new(peer_id, voting_power);
        let total_voting_power: u64 = self.validators.iter().map(|v| v.voting_power).sum();
        new_validator.priority -= self.new_joiner_penalty_factor * total_voting_power as f64;
        self.validators.insert(new_validator);
    }

    pub fn save_state(&self) -> io::Result<()> {
        let file_path = format!("{}/state.json", ROUND_ROBIN_CACHE);
        let serialized = serde_json::to_string(self).unwrap();
        let mut file = File::create(file_path)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
}

impl Drop for WeightedRoundRobin {
    fn drop(&mut self) {
        self.save_state().unwrap();
    }
}

impl PartialEq for AngstromValidator {
    fn eq(&self, other: &Self) -> bool {
        self.peer_id == other.peer_id
    }
}

impl Eq for AngstromValidator {}

impl std::hash::Hash for AngstromValidator {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.peer_id.hash(state);
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    fn cleanup(vm: WeightedRoundRobin) {
        drop(vm);
        std::fs::remove_file(format!("{}/state.json", ROUND_ROBIN_CACHE)).unwrap_or(());
    }

    #[test]
    fn test_round_robin_simulation() {
        let peers = HashMap::from([
            ("Alice".to_string(), PeerId::random()),
            ("Bob".to_string(), PeerId::random()),
            ("Charlie".to_string(), PeerId::random())
        ]);
        let validators = vec![
            AngstromValidator::new(peers["Alice"], 100),
            AngstromValidator::new(peers["Bob"], 200),
            AngstromValidator::new(peers["Charlie"], 300),
        ];
        let mut algo = WeightedRoundRobin::new(validators, BlockNumber::default(), None);

        fn simulate_rounds(algo: &mut WeightedRoundRobin, rounds: usize) -> HashMap<PeerId, usize> {
            let mut stats = HashMap::new();
            for i in 1..=rounds {
                let proposer = algo.choose_proposer(BlockNumber::from(i as u64)).unwrap();
                *stats.entry(proposer).or_insert(0) += 1;
            }
            stats
        }

        let rounds = 1000;
        let stats = simulate_rounds(&mut algo, rounds);

        assert_eq!(stats.len(), 3);

        let total_selections: usize = stats.values().sum();
        assert_eq!(total_selections, rounds);

        let alice_ratio = *stats.get(&peers["Alice"]).unwrap() as f64 / rounds as f64;
        let bob_ratio = *stats.get(&peers["Bob"]).unwrap() as f64 / rounds as f64;
        let charlie_ratio = *stats.get(&peers["Charlie"]).unwrap() as f64 / rounds as f64;

        assert!((alice_ratio - 0.167).abs() < 0.05);
        assert!((bob_ratio - 0.333).abs() < 0.05);
        assert!((charlie_ratio - 0.5).abs() < 0.05);

        // important otherwise you'd be working with cached state
        cleanup(algo);
    }

    #[test]
    fn test_add_remove_validator() {
        let peers = HashMap::from([
            ("Alice".to_string(), PeerId::random()),
            ("Bob".to_string(), PeerId::random()),
            ("Charlie".to_string(), PeerId::random())
        ]);
        let validators = vec![
            AngstromValidator::new(peers["Alice"], 100),
            AngstromValidator::new(peers["Bob"], 200),
        ];
        let mut algo = WeightedRoundRobin::new(validators, BlockNumber::default(), None);

        fn simulate_rounds(
            algo: &mut WeightedRoundRobin,
            rounds: usize,
            offset: usize
        ) -> HashMap<PeerId, usize> {
            let mut stats = HashMap::new();
            for i in offset..(offset + rounds) {
                let proposer = algo.choose_proposer(BlockNumber::from(i as u64)).unwrap();
                *stats.entry(proposer).or_insert(0) += 1;
            }
            stats
        }

        let rounds = 1000;
        let initial_stats = simulate_rounds(&mut algo, rounds, 1);
        assert_eq!(initial_stats.len(), 2);

        algo.add_validator(peers["Charlie"], 300);

        let after_add_stats = simulate_rounds(&mut algo, rounds, 1001);
        assert_eq!(after_add_stats.len(), 3);
        assert!(after_add_stats.contains_key(&peers["Charlie"]));

        algo.remove_validator(&peers["Bob"]);

        let after_remove_stats = simulate_rounds(&mut algo, rounds, 2001);
        assert_eq!(after_remove_stats.len(), 2);
        assert!(!after_remove_stats.contains_key(&peers["Bob"]));

        // important otherwise you'd be working with cached state
        cleanup(algo);
    }

    #[test]
    fn test_save_load_state() {
        let peers = HashMap::from([
            ("Alice".to_string(), PeerId::random()),
            ("Bob".to_string(), PeerId::random()),
            ("Charlie".to_string(), PeerId::random())
        ]);
        let validators = vec![
            AngstromValidator::new(peers["Alice"], 100),
            AngstromValidator::new(peers["Bob"], 200),
            AngstromValidator::new(peers["Charlie"], 300),
        ];
        let algo = WeightedRoundRobin::new(validators, BlockNumber::default(), None);

        algo.save_state().unwrap();

        let loaded_algo = WeightedRoundRobin::new(vec![], BlockNumber::default(), None);

        assert_eq!(algo.validators, loaded_algo.validators);
        assert_eq!(algo.new_joiner_penalty_factor, loaded_algo.new_joiner_penalty_factor);
        assert_eq!(algo.block_number, loaded_algo.block_number);

        // important otherwise you'd be working with cached state
        cleanup(algo);
    }
}
