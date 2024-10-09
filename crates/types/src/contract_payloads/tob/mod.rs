use std::collections::HashMap;

use alloy::primitives::{aliases::I24, U256};

use super::rewards::RewardsUpdate;
use crate::matching::uniswap::Tick;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ToBOutcome {
    pub start_tick:      i32,
    pub start_liquidity: u128,
    pub tribute:         U256,
    pub total_cost:      U256,
    pub total_reward:    U256,
    pub tick_donations:  HashMap<Tick, U256>
}

impl ToBOutcome {
    /// Sum of the donations across all ticks
    pub fn total_donations(&self) -> U256 {
        self.tick_donations
            .iter()
            .fold(U256::ZERO, |acc, (_tick, donation)| acc + donation)
    }

    /// Tick donations plus tribute to determine total value of this outcome
    pub fn total_value(&self) -> U256 {
        self.total_donations() + self.tribute
    }

    pub fn to_rewards_update(&self) -> RewardsUpdate {
        let mut donations = self.tick_donations.iter().collect::<Vec<_>>();
        // Will sort from lowest to highest (donations[0] will be the lowest tick
        // number)
        donations.sort_by_key(|f| f.0);
        // Each reward value is the cumulative sum of the rewards before it
        let quantities = donations
            .iter()
            .scan(U256::ZERO, |state, (_tick, q)| {
                *state += **q;
                Some(u128::try_from(*state).unwrap())
            })
            .collect::<Vec<_>>();
        let start_tick = I24::try_from(donations.first().map(|(a, _)| *a + 1).unwrap_or_default())
            .unwrap_or_default();
        match quantities.len() {
            0 | 1 => RewardsUpdate::CurrentOnly {
                amount: quantities.first().copied().unwrap_or_default()
            },
            _ => RewardsUpdate::MultiTick {
                start_tick,
                start_liquidity: self.start_liquidity,
                quantities
            }
        }
    }
}
