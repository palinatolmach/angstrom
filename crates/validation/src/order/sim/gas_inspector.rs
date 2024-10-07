use std::collections::HashMap;

use alloy::primitives::U256;
use reth_primitives::Address;
use revm::{Database, Inspector};

pub type GasUsed = u64;

/// the Gas Simulation Inspector allows us to define mutually exclusive ranges
/// based on the EVM program counter and will store the gas used for execution
/// in these ranges.
pub struct GasSimulationInspector {
    results:             HashMap<(usize, usize), GasUsed>,
    /// A map from start pc to end pc.
    measurement_ranges:  HashMap<usize, usize>,
    // the current start of the pc we are measuring
    in_flight:           Option<usize>,
    in_flight_start_gas: Option<u64>,
    /// the amount of instructions that have been run
    angstrom_address:    Address
}

impl GasSimulationInspector {
    pub fn new(angstrom_address: Address, measurement_ranges: HashMap<usize, usize>) -> Self {
        Self {
            results: HashMap::default(),
            measurement_ranges,
            angstrom_address,
            in_flight: None,
            in_flight_start_gas: None
        }
    }
}

impl<DB: Database> Inspector<DB> for GasSimulationInspector {
    fn step(
        &mut self,
        interp: &mut revm::interpreter::Interpreter,
        context: &mut revm::EvmContext<DB>
    ) {
        let addr = interp.contract().bytecode_address.unwrap();
        // we only want to check against angstrom PC
        if addr != self.angstrom_address {
            return
        }

        let pc = interp.program_counter();
        // if we currently have no measurements. check the next part of the range to
        // process.
        if self.in_flight.is_none() && self.measurement_ranges.contains_key(&pc) {
            self.in_flight = Some(pc);
            self.in_flight_start_gas = Some(interp.gas().spent());
        }
    }

    fn step_end(
        &mut self,
        interp: &mut revm::interpreter::Interpreter,
        context: &mut revm::EvmContext<DB>
    ) {
        let addr = interp.contract().bytecode_address.unwrap();
        if self.in_flight.is_none() && addr != self.angstrom_address {
            return
        }

        let pc = interp.program_counter();

        // check to see if we have reached the end of this measurement
        // segment.
        let end_pc = self
            .measurement_ranges
            .get(self.in_flight.as_ref().unwrap())
            .unwrap();

        // add the results
        if end_pc == &pc {
            let start_pc = self.in_flight.take().unwrap();
            let start_gas = self.in_flight_start_gas.take().unwrap();

            // ensure we only measure once.
            self.measurement_ranges.remove(&start_pc);

            let end_gas = interp.gas().spent();
            let gas_used = end_gas - start_gas;
            self.results.insert((start_pc, pc), gas_used);
        }
    }
}
