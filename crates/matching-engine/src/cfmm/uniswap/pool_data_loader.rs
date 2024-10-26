use std::{collections::HashMap, future::Future, sync::Arc};

use alloy::{
    primitives::{address, aliases::I24, Address, BlockNumber, U256},
    providers::{Network, Provider},
    sol,
    sol_types::{SolEvent, SolType},
    transports::Transport
};
use alloy_primitives::{Log, B256, I256};
use amms::errors::AMMError;
use angstrom_types::primitive::PoolId as AngstromPoolId;
use itertools::Itertools;

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IGetUniswapV3TickDataBatchRequest,
    "src/cfmm/uniswap/GetUniswapV3TickData.json"
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IGetUniswapV3PoolDataBatchRequest,
    "src/cfmm/uniswap/GetUniswapV3PoolData.json"
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IGetUniswapV4TickDataBatchRequest,
    "src/cfmm/uniswap/GetUniswapV4TickData.json"
}

sol! {
    #[allow(missing_docs)]
    #[sol(rpc)]
    IGetUniswapV4PoolDataBatchRequest,
    "src/cfmm/uniswap/GetUniswapV4PoolData.json"
}

sol! {
    struct PoolData {
        address tokenA;
        uint8 tokenADecimals;
        address tokenB;
        uint8 tokenBDecimals;
        uint128 liquidity;
        uint160 sqrtPrice;
        int24 tick;
        int24 tickSpacing;
        uint24 fee;
        int128 liquidityNet;
    }

    struct TickData {
        bool initialized;
        int24 tick;
        uint128 liquidityGross;
        int128 liquidityNet;
    }

    struct TicksWithBlock {
        TickData[] ticks;
        uint256 blockNumber;
    }
}

sol! {
    type PoolId is bytes32;

    #[derive(Debug, PartialEq, Eq)]
    contract IUniswapV4Pool {
        event Swap(PoolId indexed id, address indexed sender, int128 amount0, int128 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick, uint24 fee);
        event ModifyLiquidity(PoolId indexed id, address indexed sender, int24 tickLower, int24 tickUpper, int256 liquidityDelta, bytes32 salt);
    }

    #[derive(Debug, PartialEq, Eq)]
    contract IUniswapV3Pool {
        event Swap(address indexed sender, address indexed recipient, int256 amount0, int256 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick);
        event Burn(address indexed owner, int24 indexed tickLower, int24 indexed tickUpper, uint128 amount, uint256 amount0, uint256 amount1);
        event Mint(address sender, address indexed owner, int24 indexed tickLower, int24 indexed tickUpper, uint128 amount, uint256 amount0, uint256 amount1);
    }
}

#[derive(Debug, Clone)]
pub struct UniswapTickData {
    pub initialized:     bool,
    pub tick:            i32,
    pub liquidity_gross: u128,
    pub liquidity_net:   i128
}

#[derive(Debug, Clone)]
pub struct SwapEvent {
    pub sender:         Address,
    pub amount0:        I256,
    pub amount1:        I256,
    pub sqrt_price_x96: U256,
    pub liquidity:      u128,
    pub tick:           i32
}

#[derive(Debug, Clone)]
pub struct ModifyPositionEvent {
    pub sender:          Address,
    pub tick_lower:      i32,
    pub tick_upper:      i32,
    pub liquidity_delta: i128
}
#[derive(Default, Clone, Debug)]
pub struct DataLoader<A> {
    address: A
}

impl<A> DataLoader<A> {
    pub fn new(address: A) -> Self {
        DataLoader { address }
    }
}

pub trait PoolDataLoader<A> {
    fn load_tick_data<P: Provider<T, N>, T: Transport + Clone, N: Network>(
        &self,
        current_tick: I24,
        zero_for_one: bool,
        num_ticks: u16,
        tick_spacing: I24,
        block_number: Option<BlockNumber>,
        provider: Arc<P>
    ) -> impl Future<Output = Result<(Vec<UniswapTickData>, U256), AMMError>> + Send;

    fn address(&self) -> A;

    fn group_logs(logs: Vec<Log>) -> HashMap<A, Vec<Log>>;
    fn event_signatures() -> Vec<B256>;
    fn load_pool_data<P: Provider<T, N>, T: Transport + Clone, N: Network>(
        &self,
        block_number: Option<BlockNumber>,
        provider: Arc<P>
    ) -> impl Future<Output = Result<PoolData, AMMError>> + Send;

    fn is_swap_event(log: &Log) -> bool;
    fn is_modify_position_event(log: &Log) -> bool;
    fn decode_swap_event(log: &Log) -> Result<SwapEvent, alloy::sol_types::Error>;
    fn decode_modify_position_event(
        log: &Log
    ) -> Result<ModifyPositionEvent, alloy::sol_types::Error>;
}

impl PoolDataLoader<Address> for DataLoader<Address> {
    async fn load_tick_data<P: Provider<T, N>, T: Transport + Clone, N: Network>(
        &self,
        current_tick: I24,
        zero_for_one: bool,
        num_ticks: u16,
        tick_spacing: I24,
        block_number: Option<BlockNumber>,
        provider: Arc<P>
    ) -> Result<(Vec<UniswapTickData>, U256), AMMError> {
        let deployer = IGetUniswapV3TickDataBatchRequest::deploy_builder(
            provider.clone(),
            self.address,
            zero_for_one,
            current_tick,
            num_ticks,
            tick_spacing
        );

        let data = match block_number {
            Some(number) => deployer.block(number.into()).call_raw().await?,
            None => deployer.call_raw().await?
        };

        let result = TicksWithBlock::abi_decode(&data, true)?;

        let tick_data: Vec<UniswapTickData> = result
            .ticks
            .iter()
            .map(|tick| UniswapTickData {
                initialized:     tick.initialized,
                tick:            tick.tick.as_i32(),
                liquidity_gross: tick.liquidityGross,
                liquidity_net:   tick.liquidityNet
            })
            .collect();
        Ok((tick_data, result.blockNumber))
    }

    async fn load_pool_data<P: Provider<T, N>, T: Transport + Clone, N: Network>(
        &self,
        block_number: Option<BlockNumber>,
        provider: Arc<P>
    ) -> Result<PoolData, AMMError> {
        let deployer = IGetUniswapV3PoolDataBatchRequest::deploy_builder(provider, self.address);
        let res = if let Some(block_number) = block_number {
            deployer.block(block_number.into()).call_raw().await?
        } else {
            deployer.call_raw().await?
        };

        let pool_data = PoolData::abi_decode(&res, true)?;
        Ok(pool_data)
    }

    fn address(&self) -> Address {
        self.address
    }

    fn group_logs(logs: Vec<Log>) -> HashMap<Address, Vec<Log>> {
        logs.into_iter()
            .map(|log| (log.address, log))
            .into_group_map()
    }

    fn event_signatures() -> Vec<B256> {
        vec![
            IUniswapV3Pool::Swap::SIGNATURE_HASH,
            IUniswapV3Pool::Mint::SIGNATURE_HASH,
            IUniswapV3Pool::Burn::SIGNATURE_HASH,
        ]
    }

    fn is_swap_event(log: &Log) -> bool {
        log.topics()[0] == IUniswapV3Pool::Swap::SIGNATURE_HASH
    }

    fn is_modify_position_event(log: &Log) -> bool {
        log.topics()[0] == IUniswapV3Pool::Mint::SIGNATURE_HASH
            || log.topics()[0] == IUniswapV3Pool::Burn::SIGNATURE_HASH
    }

    fn decode_swap_event(log: &Log) -> Result<SwapEvent, alloy::sol_types::Error> {
        let swap_event = IUniswapV3Pool::Swap::decode_log(log, true)?;
        Ok(SwapEvent {
            sender:         swap_event.sender,
            amount0:        swap_event.amount0,
            amount1:        swap_event.amount1,
            sqrt_price_x96: U256::from(swap_event.sqrtPriceX96),
            liquidity:      swap_event.liquidity,
            tick:           swap_event.tick.as_i32()
        })
    }

    fn decode_modify_position_event(
        log: &Log
    ) -> Result<ModifyPositionEvent, alloy::sol_types::Error> {
        if log.topics()[0] == IUniswapV3Pool::Mint::SIGNATURE_HASH {
            let mint_event = IUniswapV3Pool::Mint::decode_log(log, true)?;
            Ok(ModifyPositionEvent {
                sender:          mint_event.sender,
                tick_lower:      mint_event.tickLower.as_i32(),
                tick_upper:      mint_event.tickUpper.as_i32(),
                liquidity_delta: mint_event.amount as i128
            })
        } else {
            let burn_event = IUniswapV3Pool::Burn::decode_log(log, true)?;
            Ok(ModifyPositionEvent {
                sender:          burn_event.owner,
                tick_lower:      burn_event.tickLower.as_i32(),
                tick_upper:      burn_event.tickUpper.as_i32(),
                liquidity_delta: -(burn_event.amount as i128)
            })
        }
    }
}

impl DataLoader<AngstromPoolId> {
    fn pool_manager(&self) -> Address {
        address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640")
    }
}

impl PoolDataLoader<AngstromPoolId> for DataLoader<AngstromPoolId> {
    async fn load_tick_data<P: Provider<T, N>, T: Transport + Clone, N: Network>(
        &self,
        current_tick: I24,
        zero_for_one: bool,
        num_ticks: u16,
        tick_spacing: I24,
        block_number: Option<BlockNumber>,
        provider: Arc<P>
    ) -> Result<(Vec<UniswapTickData>, U256), AMMError> {
        todo!()
    }

    fn address(&self) -> AngstromPoolId {
        self.address
    }

    fn group_logs(logs: Vec<Log>) -> HashMap<AngstromPoolId, Vec<Log>> {
        logs.into_iter()
            .filter_map(|log| {
                if Self::is_modify_position_event(&log) {
                    let modify_event =
                        IUniswapV4Pool::ModifyLiquidity::decode_log(&log, true).ok()?;
                    return Some((modify_event.id, log))
                } else if Self::is_swap_event(&log) {
                    let swap = IUniswapV4Pool::Swap::decode_log(&log, true).ok()?;
                    return Some((swap.id, log))
                };
                None
            })
            .into_group_map()
    }

    fn event_signatures() -> Vec<B256> {
        vec![IUniswapV4Pool::Swap::SIGNATURE_HASH, IUniswapV4Pool::ModifyLiquidity::SIGNATURE_HASH]
    }

    async fn load_pool_data<P: Provider<T, N>, T: Transport + Clone, N: Network>(
        &self,
        block_number: Option<BlockNumber>,
        provider: Arc<P>
    ) -> Result<PoolData, AMMError> {
        // TODO: pass the param
        let deployer =
            IGetUniswapV3PoolDataBatchRequest::deploy_builder(provider, self.pool_manager());
        let res = if let Some(block_number) = block_number {
            deployer.block(block_number.into()).call_raw().await?
        } else {
            deployer.call_raw().await?
        };

        let pool_data = PoolData::abi_decode(&res, true)?;
        Ok(pool_data)
    }

    fn is_swap_event(log: &Log) -> bool {
        log.topics()[0] == IUniswapV4Pool::Swap::SIGNATURE_HASH
    }

    fn is_modify_position_event(log: &Log) -> bool {
        log.topics()[0] == IUniswapV4Pool::ModifyLiquidity::SIGNATURE_HASH
    }

    fn decode_swap_event(log: &Log) -> Result<SwapEvent, alloy::sol_types::Error> {
        let swap_event = IUniswapV4Pool::Swap::decode_log(log, true)?;
        Ok(SwapEvent {
            sender:         swap_event.sender,
            amount0:        i128_to_i256(swap_event.amount0),
            amount1:        i128_to_i256(swap_event.amount1),
            sqrt_price_x96: U256::from(swap_event.sqrtPriceX96),
            liquidity:      swap_event.liquidity,
            tick:           swap_event.tick.as_i32()
        })
    }

    fn decode_modify_position_event(
        log: &Log
    ) -> Result<ModifyPositionEvent, alloy::sol_types::Error> {
        let modify_event = IUniswapV4Pool::ModifyLiquidity::decode_log(log, true)?;
        Ok(ModifyPositionEvent {
            sender:          modify_event.sender,
            tick_lower:      modify_event.tickLower.as_i32(),
            tick_upper:      modify_event.tickUpper.as_i32(),
            // TODO: remove downcast
            liquidity_delta: 0 // modify_event.liquidityDelta.as_i28(),
        })
    }
}

fn i128_to_i256(value: i128) -> I256 {
    let mut bytes = [0u8; 32];
    let value_bytes = value.to_be_bytes();
    bytes[16..].copy_from_slice(&value_bytes);
    I256::from_be_bytes(bytes)
}
