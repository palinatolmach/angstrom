#!/bin/bash
# The working directory should always be the same as the script's directory
cd "$(dirname "$0")"

# Create forge bindings as a module in the types crate with a regex of the particular contracts we actually need bound
forge bind -b ../crates/types/src/contract_bindings --select "^(Angstrom|PoolManager|PoolGate|MockRewardsManager|MintableMockERC20)$" --alloy --module --skip-cargo-toml --overwrite