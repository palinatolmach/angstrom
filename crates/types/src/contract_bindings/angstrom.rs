pub use alloy_primitives::*;
use alloy_rlp::{Decodable, Encodable, Error};
use alloy_rlp_derive::{RlpDecodable, RlpEncodable};
use alloy_sol_macro::sol;
use serde::{Deserialize, Serialize};

sol! {
    #![sol(all_derives = true)]

    interface Angstrom {
        event OwnershipHandoverCanceled(address indexed pendingOwner);
        event OwnershipHandoverRequested(address indexed pendingOwner);
        event OwnershipTransferred(address indexed oldOwner, address indexed newOwner);

        type Currency is address;

        struct Bundle {
            ExecutedOrder[] orders;
            bytes uniswapData;
        }

        #[derive(RlpEncodable, RlpDecodable)]
        struct ExecutedOrder {
            Order order;
            bytes signature;
            uint256 amountInActual;
            uint256 amountOutActual;
        }

        struct SwapParams {
            bool zeroForOne;
            int256 amountSpecified;
            uint160 sqrtPriceLimitX96;
        }

        #[derive(Serialize, Deserialize, RlpEncodable, RlpDecodable)]
        struct Order {
            uint256 nonce;
            uint8 orderType;
            address currencyIn;
            address currencyOut;
            uint128 amountIn;
            uint128 amountOutMin;
            uint256 deadline;
            bytes preHook;
            bytes postHook;
        }

        enum OrderType {
            User,
            Searcher,
            Limit,
            UserFallible,
            SearcherFallible
        }

        #[derive(Serialize, Deserialize)]
        struct PoolKey {
            address currency0;
            address currency1;
            uint24 fee;
            int24 tickSpacing;
            address hooks;
        }

        /// @notice Uniswap instructions to execute after lock is taken.
        struct UniswapData {
            /// @member The discrete swaps to perform, there should be at most one entry
            ///         per pool.
            PoolSwap[] swaps;
            /// @member The currency settlements to perform, there should be at most one
            ///         entry per currency.
            CurrencySettlement[] currencies;
            /// @member The fees to pay to each pool, there should be at most one entry
            ///         per pool.
            PoolFees[] pools;
        }

        /// @notice Instruction to execute a swap on UniswapV4.
        struct PoolSwap {
            /// @member The pool to perform the swap on.
            PoolKey pool;
            /// @member The input currency.
            Currency currencyIn;
            /// @member The amount of input.
            uint256 amountIn;
        }

        /// @notice Instruction to settle an amount of currency.
        struct CurrencySettlement {
            /// @member The currency to settle.
            Currency currency;
            /// @member The amount to settle, positive indicates we must pay, negative
            ///         indicates we are paid.
            int256 amountNet;
        }

        /// @notice Instruction to donate revenue to a pool.
        struct PoolFees {
            /// @member The pool to pay fees to.
            PoolKey pool;
            /// @member The amount0 fee.
            uint256 fees0;
            /// @member The amount1 fee.
            uint256 fees1;
        }


        function beforeSwap(address aSender, SwapParams memory, SwapParams memory, bytes memory)
            external
            view
            returns (bytes4 rSelector);
        function cancelOwnershipHandover() external payable;
        function claimFees(address aCurrency, address aRecipient) external;
        function completeOwnershipHandover(address pendingOwner) external payable;
        function eip712Domain()
            external
            view
            returns (
                bytes1 fields,
                string memory name,
                string memory version,
                uint256 chainId,
                address verifyingContract,
                bytes32 salt,
                uint256[] memory extensions
            );
        function invalidateUnorderedNonces(uint256 wordPos, uint256 mask) external;
        function lockAcquired(bytes memory aBundle) external returns (bytes memory);
        function nonceBitmap(address, uint256) external view returns (uint256);
        function owner() external view returns (address result);
        function ownershipHandoverExpiresAt(
            address pendingOwner
        ) external view returns (uint256 result);
        function poolManager() external view returns (address);
        function process(Bundle memory aBundle) external;
        function renounceOwnership() external payable;
        function requestOwnershipHandover() external payable;
        function transferOwnership(address newOwner) external payable;
    }
}

impl Encodable for Angstrom::PoolKey {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        self.currency0.encode(out);
        self.currency1.encode(out);
        self.fee.encode(out);

        if self.tickSpacing.is_negative() {
            1_u8.encode(out);
            let spacing = (!self.tickSpacing).overflowing_add(1).0 as u32;
            spacing.encode(out);
        } else {
            0_u8.encode(out);
            (self.tickSpacing as u32).encode(out);
        }

        self.hooks.encode(out);
    }

    fn length(&self) -> usize {
        68
    }
}

impl Decodable for Angstrom::PoolKey {
    fn decode(buf: &mut &[u8]) -> alloy_rlp::Result<Self> {
        let cur_0 = Address::decode(buf)?;
        let cur_1 = Address::decode(buf)?;
        let fee = u32::decode(buf)?;
        let is_neg: bool = Decodable::decode(buf)?;

        let tick_spacing = if is_neg {
            let spacing = u32::decode(buf)?;
            (!spacing).overflowing_add(1).0 as i32
        } else {
            u32::decode(buf)? as i32
        };
        let hooks = Address::decode(buf)?;

        Ok(Self { hooks, fee, tickSpacing: tick_spacing, currency0: cur_0, currency1: cur_1 })
    }
}
