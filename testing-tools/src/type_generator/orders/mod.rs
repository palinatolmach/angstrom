use alloy_primitives::{Address, FixedBytes, Uint, U256};
use angstrom_types::{
    matching::Ray,
    orders::{OrderId, OrderPriorityData},
    primitive::PoolId,
    sol_bindings::{
        ext::RawPoolOrder,
        grouped_orders::{
            FlashVariants, GroupedVanillaOrder, OrderWithStorageData, StandingVariants
        },
        rpc_orders::{
            ExactFlashOrder, ExactStandingOrder, PartialFlashOrder, PartialStandingOrder,
            TopOfBlockOrder
        }
    }
};
use rand::{rngs::ThreadRng, Rng};
use rand_distr::{num_traits::ToPrimitive, Distribution, SkewNormal};

// mod stored;

// fn build_priority_data(order: &GroupedVanillaOrder) -> OrderPriorityData {
//     OrderPriorityData { price: order.price().into(), volume: order.quantity()
// as u128, gas: 10 } }

#[derive(Clone, Debug, Default)]
pub struct UserOrderBuilder {
    /// If the order is not a Standing order, it is KillOrFill
    is_standing: bool,
    /// If the order is not an Exact order, it is Partial
    is_exact:    bool,
    block:       u64,
    nonce:       u64,
    recipient:   Address,
    asset_in:    Address,
    asset_out:   Address,
    amount:      u128,
    min_price:   Ray
}

impl UserOrderBuilder {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn standing(self) -> Self {
        Self { is_standing: true, ..self }
    }

    pub fn kill_or_fill(self) -> Self {
        Self { is_standing: false, ..self }
    }

    /// Sets the order to be kill-or-fill or standing by explicit boolean
    pub fn is_standing(self, is_standing: bool) -> Self {
        Self { is_standing, ..self }
    }

    pub fn exact(self) -> Self {
        Self { is_exact: true, ..self }
    }

    pub fn partial(self) -> Self {
        Self { is_exact: false, ..self }
    }

    /// Sets the order to be exact or partial by explicit boolean
    pub fn is_exact(self, is_exact: bool) -> Self {
        Self { is_exact, ..self }
    }

    pub fn block(self, block: u64) -> Self {
        Self { block, ..self }
    }

    pub fn nonce(self, nonce: u64) -> Self {
        Self { nonce, ..self }
    }

    pub fn recipient(self, recipient: Address) -> Self {
        Self { recipient, ..self }
    }

    pub fn asset_in(self, asset_in: Address) -> Self {
        Self { asset_in, ..self }
    }

    pub fn asset_out(self, asset_out: Address) -> Self {
        Self { asset_out, ..self }
    }

    pub fn amount(self, amount: u128) -> Self {
        Self { amount, ..self }
    }

    pub fn min_price(self, min_price: Ray) -> Self {
        Self { min_price, ..self }
    }

    pub fn build(self) -> GroupedVanillaOrder {
        match (self.is_standing, self.is_exact) {
            (true, true) => {
                let order = ExactStandingOrder {
                    assetIn: self.asset_in,
                    assetOut: self.asset_out,
                    amount: self.amount,
                    minPrice: *self.min_price,
                    recipient: self.recipient,
                    nonce: self.nonce,
                    ..Default::default()
                };
                GroupedVanillaOrder::Standing(StandingVariants::Exact(order))
            }
            (true, false) => {
                let order = PartialStandingOrder {
                    assetIn: self.asset_in,
                    assetOut: self.asset_out,
                    maxAmountIn: self.amount,
                    minPrice: *self.min_price,
                    recipient: self.recipient,
                    ..Default::default()
                };
                GroupedVanillaOrder::Standing(StandingVariants::Partial(order))
            }
            (false, true) => {
                let order = ExactFlashOrder {
                    validForBlock: self.block,
                    assetIn: self.asset_in,
                    assetOut: self.asset_out,
                    amount: self.amount,
                    minPrice: *self.min_price,
                    recipient: self.recipient,
                    ..Default::default()
                };
                GroupedVanillaOrder::KillOrFill(FlashVariants::Exact(order))
            }
            (false, false) => {
                let order = PartialFlashOrder {
                    validForBlock: self.block,
                    assetIn: self.asset_in,
                    assetOut: self.asset_out,
                    maxAmountIn: self.amount,
                    minPrice: *self.min_price,
                    recipient: self.recipient,
                    ..Default::default()
                };
                GroupedVanillaOrder::KillOrFill(FlashVariants::Partial(order))
            }
        }
    }

    /// Lets us chain right into the storage wrapper
    pub fn with_storage(self) -> StoredOrderBuilder {
        StoredOrderBuilder::new(self.build())
    }
}

#[derive(Clone, Debug)]
pub struct StoredOrderBuilder {
    order:       GroupedVanillaOrder,
    is_bid:      bool,
    pool_id:     Option<FixedBytes<32>>,
    valid_block: Option<u64>,
    tob_reward:  Option<U256>
}

impl StoredOrderBuilder {
    pub fn new(order: GroupedVanillaOrder) -> Self {
        Self { order, is_bid: false, pool_id: None, valid_block: None, tob_reward: None }
    }

    pub fn from_builder(user_order: UserOrderBuilder) -> Self {
        Self::new(user_order.build())
    }

    pub fn bid(self) -> Self {
        Self { is_bid: true, ..self }
    }

    pub fn ask(self) -> Self {
        Self { is_bid: false, ..self }
    }

    pub fn is_bid(self, is_bid: bool) -> Self {
        // Can this be determined from the order and the pool?  Maybe...
        Self { is_bid, ..self }
    }

    pub fn pool_id(self, pool_id: FixedBytes<32>) -> Self {
        Self { pool_id: Some(pool_id), ..self }
    }

    pub fn valid_block(self, valid_block: u64) -> Self {
        Self { valid_block: Some(valid_block), ..self }
    }

    pub fn tob_reward(self, tob_reward: U256) -> Self {
        Self { tob_reward: Some(tob_reward), ..self }
    }

    pub fn build(self) -> OrderWithStorageData<GroupedVanillaOrder> {
        let is_bid = self.is_bid;
        let pool_id = self.pool_id.unwrap_or_default();
        let order_id = OrderIdBuilder::new()
            .pool_id(pool_id)
            .order_hash(self.order.hash())
            .build();
        // Our specified block or the order's specified block or default
        let valid_block = self
            .valid_block
            .or(self.order.flash_block())
            .unwrap_or_default();
        let priority_data = OrderPriorityData {
            price:  self.order.price().into(),
            volume: self.order.quantity().to(),
            gas:    0
        };
        let tob_reward = self.tob_reward.unwrap_or_default();
        OrderWithStorageData {
            invalidates: vec![],
            order: self.order,
            priority_data,
            is_bid,
            is_currently_valid: true,
            is_valid: true,
            order_id,
            pool_id,
            valid_block,
            tob_reward
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct OrderIdBuilder {
    address:    Option<Address>,
    pool_id:    Option<PoolId>,
    order_hash: Option<FixedBytes<32>>
}

impl OrderIdBuilder {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn address(self, address: Address) -> Self {
        Self { address: Some(address), ..self }
    }

    pub fn pool_id(self, pool_id: PoolId) -> Self {
        Self { pool_id: Some(pool_id), ..self }
    }

    pub fn order_hash(self, order_hash: FixedBytes<32>) -> Self {
        Self { order_hash: Some(order_hash), ..self }
    }

    pub fn build(self) -> OrderId {
        let address = self.address.unwrap_or_default();
        let pool_id = self.pool_id.unwrap_or_default();
        let hash = self.order_hash.unwrap_or_default();
        OrderId {
            address,
            pool_id,
            hash,
            flash_block: None,
            location: Default::default(),
            deadline: None,
            reuse_avoidance: angstrom_types::sol_bindings::RespendAvoidanceMethod::Block(0)
        }
    }
}

pub fn generate_top_of_block_order(
    rng: &mut ThreadRng,
    is_bid: bool,
    pool_id: Option<PoolId>,
    valid_block: Option<u64>,
    quantity_in: Option<u128>,
    quantity_out: Option<u128>
) -> OrderWithStorageData<TopOfBlockOrder> {
    let pool_id = pool_id.unwrap_or_default();
    let valid_block = valid_block.unwrap_or_default();
    // Could update this to be within a distribution
    let price: u128 = rng.gen();
    let volume: u128 = rng.gen();
    let gas: u128 = rng.gen();
    let order =
        build_top_of_block_order(quantity_in.unwrap_or_default(), quantity_out.unwrap_or_default());

    let priority_data = OrderPriorityData { price: U256::from(price), volume, gas };
    let order_id = OrderIdBuilder::new()
        .pool_id(pool_id)
        .order_hash(order.order_hash())
        .build();
    // Todo: Sign It, make this overall better
    // StoredOrderBuilder::new(order).is_bid(is_bid).valid_block(valid_block).
    // pool_id(pool_id).build();
    OrderWithStorageData {
        invalidates: vec![],
        order,
        priority_data,
        is_bid,
        is_currently_valid: true,
        is_valid: true,
        order_id,
        pool_id,
        valid_block,
        tob_reward: U256::ZERO
    }
}

pub fn build_top_of_block_order(quantity_in: u128, quantity_out: u128) -> TopOfBlockOrder {
    TopOfBlockOrder { quantityIn: quantity_in, quantityOut: quantity_out, ..Default::default() }
}

#[derive(Debug, Default)]
pub struct DistributionParameters {
    pub location: f64,
    pub scale:    f64,
    pub shape:    f64
}

impl DistributionParameters {
    pub fn crossed_at(location: f64) -> (Self, Self) {
        let bids = Self { location, scale: 100000.0, shape: -2.0 };
        let asks = Self { location, scale: 100000.0, shape: 2.0 };

        (bids, asks)
    }

    pub fn fixed_at(location: f64) -> (Self, Self) {
        let bids = Self { location, scale: 1.0, shape: 0.0 };
        let asks = Self { location, scale: 1.0, shape: 0.0 };

        (bids, asks)
    }
}

pub fn generate_order_distribution(
    is_bid: bool,
    order_count: usize,
    priceparams: DistributionParameters,
    volumeparams: DistributionParameters,
    pool_id: PoolId,
    valid_block: u64
) -> Result<Vec<OrderWithStorageData<GroupedVanillaOrder>>, String> {
    let DistributionParameters { location: price_location, scale: price_scale, shape: price_shape } =
        priceparams;
    let DistributionParameters { location: v_location, scale: v_scale, shape: v_shape } =
        volumeparams;
    let mut rng = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    let price_gen = SkewNormal::new(price_location, price_scale, price_shape)
        .map_err(|e| format!("Error creating price distribution: {}", e))?;
    let volume_gen = SkewNormal::new(v_location, v_scale, v_shape)
        .map_err(|e| format!("Error creating price distribution: {}", e))?;
    Ok(price_gen
        .sample_iter(&mut rng)
        .zip(volume_gen.sample_iter(&mut rng2))
        .map(|(p, v)| {
            UserOrderBuilder::new()
                .is_standing(false)
                .block(valid_block)
                .amount(v.to_u128().unwrap_or_default())
                .min_price(Ray::from(Uint::from(p.to_u128().unwrap_or_default())))
                .with_storage()
                .pool_id(pool_id)
                .is_bid(is_bid)
                .valid_block(valid_block)
                .build()
        })
        .take(order_count)
        .collect())
}
