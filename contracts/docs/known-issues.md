- `PoolConfigStore` does not check that `tickSpacing < MAX`
- `_decodeAndReward` may DoS if trying to reward far out ticks => not a major attack as reward
algorithm is *generally* correlated to ticks swapped across

### Bundle Building Footguns
- Liquidity positions do not include the upper bound, donating to the upper bound will not credit
positions which do not otherwise encompass that tick and may even burn the tokens if you donate to
the highest uninitialized bound.
- Can also burn funds by donating to the current tick when liquidity is zero.
