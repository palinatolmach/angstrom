use alloy::contract::CallBuilder;
use eyre::eyre;

pub mod anvil;
pub mod deploy;
pub mod environment;
//mod reward;
//pub use reward::RewardTestEnv;

/// This trait is used to provide safe run and potentially debug capabilities
/// for our local contract runs.
pub trait DebugTransaction {
    #[allow(async_fn_in_trait)] // OK because this is not for public consumption
    async fn run_safe(self) -> eyre::Result<()>;
}

impl<T, P, D> DebugTransaction for CallBuilder<T, P, D>
where
    T: Clone + Send + Sync + alloy::transports::Transport,
    P: alloy::providers::Provider<T>,
    D: alloy::contract::CallDecoder
{
    async fn run_safe(self) -> eyre::Result<()> {
        let receipt = self
            .gas(30_000_000_u128)
            .send()
            .await?
            .get_receipt()
            .await?;
        if receipt.inner.status() {
            Ok(())
        } else {
            // We can make this do a cool backtrace later
            Err(eyre!("Transaction with hash {} failed", receipt.transaction_hash))
        }
    }
}
