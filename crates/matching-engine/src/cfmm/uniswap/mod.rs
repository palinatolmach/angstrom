// uint 160 for represending SqrtPriceX96

pub mod pool;
pub mod pool_manager;
pub mod pool_providers;
pub mod tob;

type Tick = i32;

#[cfg(test)]
mod tests {
    use alloy::primitives::U160;

    use super::*;

    // #[test]
    // fn requires_contiguous_ticks() {
    //     let good_ranges = vec![
    //         PoolRange::new(2000, 2100, 10000000).unwrap(),
    //         PoolRange::new(2100, 2200, 10000000).unwrap(),
    //         PoolRange::new(2200, 2300, 10000000).unwrap(),
    //         PoolRange::new(2300, 2400, 10000000).unwrap(),
    //         PoolRange::new(2400, 2500, 10000000).unwrap(),
    //     ];

    //     let bad_ranges = vec![
    //         PoolRange::new(2000, 2100, 10000000).unwrap(),
    //         PoolRange::new(2100, 2200, 10000000).unwrap(),
    //         PoolRange::new(2210, 2300, 10000000).unwrap(),
    //         PoolRange::new(2300, 2400, 10000000).unwrap(),
    //         PoolRange::new(2400, 2500, 10000000).unwrap(),
    //     ];

    //     let valid_price =
    // SqrtPriceX96::from(get_sqrt_ratio_at_tick(2325).unwrap());

    //     // Good ranges, good price, should be fine
    //     MarketSnapshot::new(good_ranges.clone(), valid_price).unwrap();
    //     // Good ranges, bad price, should fail
    //     assert!(MarketSnapshot::new(good_ranges, U160::from(0).into()).is_err());
    //     // Bad ranges, good price, should fail
    //     assert!(MarketSnapshot::new(bad_ranges, U160::from(0).into()).is_err());
    // }

    #[test]
    fn span_sums_and_rounding_work() {
        let liq = 50000000000;
        let t1 = uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(10).unwrap();
        let t2 = uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(20).unwrap();
        let t3 = uniswap_v3_math::tick_math::get_sqrt_ratio_at_tick(30).unwrap();

        let step_12 =
            uniswap_v3_math::sqrt_price_math::_get_amount_0_delta(t1, t2, liq, true).unwrap();
        let step_23 =
            uniswap_v3_math::sqrt_price_math::_get_amount_0_delta(t2, t3, liq, true).unwrap();
        let step_13 =
            uniswap_v3_math::sqrt_price_math::_get_amount_0_delta(t1, t3, liq, true).unwrap();

        assert_eq!(step_12 + step_23, step_13, "Sums not equal");
    }

    #[test]
    fn test_ask_iter() {}
}
