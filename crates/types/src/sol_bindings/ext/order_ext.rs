use super::FlipOrder;
use crate::sol_bindings::rpc_orders::{
    ExactFlashOrder, ExactStandingOrder, PartialFlashOrder, PartialStandingOrder
};
impl FlipOrder for PartialFlashOrder {
    fn flip_order(&self) -> Self {
        let mut this = self.clone();
        // swap assets
        std::mem::swap(&mut this.assetIn, &mut this.assetOut);
        /// adjust amount to other side
        this.minAmountIn *= this.minPrice;
        this.maxAmountIn *= this.minPrice;
    }
}

impl FlipOrder for ExactFlashOrder {
    fn flip_order(&self) -> Self {
        let mut this = self.clone();
        // swap assets
        std::mem::swap(&mut this.assetIn, &mut this.assetOut);
        /// adjust amount to other side
        this.minAmountIn *= this.minPrice;
        this.maxAmountIn *= this.minPrice;
    }
}

impl FlipOrder for PartialStandingOrder {
    fn flip_order(&self) -> Self {}
}

impl FlipOrder for ExactStandingOrder {
    fn flip_order(&self) -> Self {}
}
