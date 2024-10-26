use alloy_primitives::{aliases::I24, I256};

pub mod pool;
pub mod pool_data_loader;
pub mod pool_manager;
pub mod pool_providers;
pub mod tob;

fn i32_to_i24(val: i32) -> I24 {
    let mut bytes = [0u8; 3];
    let value_bytes = val.to_be_bytes();
    bytes[..].copy_from_slice(&value_bytes[1..]);
    I24::from_be_bytes(bytes)
}

fn i128_to_i256(value: i128) -> I256 {
    let mut bytes = [0u8; I256::BYTES];
    let value_bytes = value.to_be_bytes();
    bytes[16..].copy_from_slice(&value_bytes);
    I256::from_be_bytes(bytes)
}

fn i256_to_i128(value: I256) -> i128 {
    let mut bytes = [0u8; 16];
    let value_bytes: [u8; I256::BYTES] = value.to_be_bytes();
    bytes.copy_from_slice(&value_bytes[16..]);
    i128::from_be_bytes(bytes)
}

#[cfg(test)]
mod tests {

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
