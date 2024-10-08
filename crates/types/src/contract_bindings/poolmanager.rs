///Module containing a contract's types and functions.
/**

```solidity
library IPoolManager {
    struct ModifyLiquidityParams { int24 tickLower; int24 tickUpper; int256 liquidityDelta; bytes32 salt; }
    struct SwapParams { bool zeroForOne; int256 amountSpecified; uint160 sqrtPriceLimitX96; }
}
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod IPoolManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /**```solidity
struct ModifyLiquidityParams { int24 tickLower; int24 tickUpper; int256 liquidityDelta; bytes32 salt; }
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ModifyLiquidityParams {
        pub tickLower: alloy::sol_types::private::primitives::aliases::I24,
        pub tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        pub liquidityDelta: alloy::sol_types::private::primitives::aliases::I256,
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Int<256>,
            alloy::sol_types::sol_data::FixedBytes<32>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::I24,
            alloy::sol_types::private::primitives::aliases::I24,
            alloy::sol_types::private::primitives::aliases::I256,
            alloy::sol_types::private::FixedBytes<32>,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ModifyLiquidityParams> for UnderlyingRustTuple<'_> {
            fn from(value: ModifyLiquidityParams) -> Self {
                (value.tickLower, value.tickUpper, value.liquidityDelta, value.salt)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ModifyLiquidityParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    tickLower: tuple.0,
                    tickUpper: tuple.1,
                    liquidityDelta: tuple.2,
                    salt: tuple.3,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for ModifyLiquidityParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for ModifyLiquidityParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickLower),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickUpper),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityDelta),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for ModifyLiquidityParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for ModifyLiquidityParams {
            const NAME: &'static str = "ModifyLiquidityParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "ModifyLiquidityParams(int24 tickLower,int24 tickUpper,int256 liquidityDelta,bytes32 salt)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tickLower)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tickUpper)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.liquidityDelta,
                        )
                        .0,
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.salt)
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for ModifyLiquidityParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tickLower,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tickUpper,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.liquidityDelta,
                    )
                    + <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.salt)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tickLower,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tickUpper,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.liquidityDelta,
                    out,
                );
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.salt,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**```solidity
struct SwapParams { bool zeroForOne; int256 amountSpecified; uint160 sqrtPriceLimitX96; }
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct SwapParams {
        pub zeroForOne: bool,
        pub amountSpecified: alloy::sol_types::private::primitives::aliases::I256,
        pub sqrtPriceLimitX96: alloy::sol_types::private::primitives::aliases::U160,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Bool,
            alloy::sol_types::sol_data::Int<256>,
            alloy::sol_types::sol_data::Uint<160>,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            bool,
            alloy::sol_types::private::primitives::aliases::I256,
            alloy::sol_types::private::primitives::aliases::U160,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SwapParams> for UnderlyingRustTuple<'_> {
            fn from(value: SwapParams) -> Self {
                (value.zeroForOne, value.amountSpecified, value.sqrtPriceLimitX96)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SwapParams {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    zeroForOne: tuple.0,
                    amountSpecified: tuple.1,
                    sqrtPriceLimitX96: tuple.2,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for SwapParams {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for SwapParams {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.zeroForOne,
                    ),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amountSpecified),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.sqrtPriceLimitX96),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for SwapParams {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for SwapParams {
            const NAME: &'static str = "SwapParams";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "SwapParams(bool zeroForOne,int256 amountSpecified,uint160 sqrtPriceLimitX96)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::eip712_data_word(
                            &self.zeroForOne,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.amountSpecified,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::eip712_data_word(
                            &self.sqrtPriceLimitX96,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for SwapParams {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.zeroForOne,
                    )
                    + <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.amountSpecified,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.sqrtPriceLimitX96,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <alloy::sol_types::sol_data::Bool as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.zeroForOne,
                    out,
                );
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.amountSpecified,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    160,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.sqrtPriceLimitX96,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`IPoolManager`](self) contract instance.

See the [wrapper's documentation](`IPoolManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> IPoolManagerInstance<T, P, N> {
        IPoolManagerInstance::<T, P, N>::new(address, provider)
    }
    /**A [`IPoolManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IPoolManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct IPoolManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for IPoolManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("IPoolManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IPoolManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`IPoolManager`](self) contract instance.

See the [wrapper's documentation](`IPoolManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> IPoolManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> IPoolManagerInstance<T, P, N> {
            IPoolManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IPoolManagerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > IPoolManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
    }
}
/**

Generated by the following Solidity interface...
```solidity
library IPoolManager {
    struct ModifyLiquidityParams {
        int24 tickLower;
        int24 tickUpper;
        int256 liquidityDelta;
        bytes32 salt;
    }
    struct SwapParams {
        bool zeroForOne;
        int256 amountSpecified;
        uint160 sqrtPriceLimitX96;
    }
}

interface PoolManager {
    type BalanceDelta is int256;
    type Currency is address;
    type PoolId is bytes32;
    struct PoolKey {
        Currency currency0;
        Currency currency1;
        uint24 fee;
        int24 tickSpacing;
        address hooks;
    }

    error AlreadyUnlocked();
    error ContractUnlocked();
    error CurrenciesOutOfOrderOrEqual(address currency0, address currency1);
    error CurrencyNotSettled();
    error DelegateCallNotAllowed();
    error InvalidBips();
    error InvalidCaller();
    error ManagerLocked();
    error MustClearExactPositiveDelta();
    error NonzeroNativeValue();
    error PoolNotInitialized();
    error ProtocolFeeCannotBeFetched();
    error ProtocolFeeTooLarge(uint24 fee);
    error SwapAmountCannotBeZero();
    error TickSpacingTooLarge(int24 tickSpacing);
    error TickSpacingTooSmall(int24 tickSpacing);
    error UnauthorizedDynamicLPFeeUpdate();

    event Approval(address indexed owner, address indexed spender, uint256 indexed id, uint256 amount);
    event Donate(PoolId indexed id, address indexed sender, uint256 amount0, uint256 amount1);
    event Initialize(PoolId indexed id, Currency indexed currency0, Currency indexed currency1, uint24 fee, int24 tickSpacing, address hooks, uint160 sqrtPriceX96, int24 tick);
    event ModifyLiquidity(PoolId indexed id, address indexed sender, int24 tickLower, int24 tickUpper, int256 liquidityDelta, bytes32 salt);
    event OperatorSet(address indexed owner, address indexed operator, bool approved);
    event OwnershipTransferred(address indexed user, address indexed newOwner);
    event ProtocolFeeControllerUpdated(address indexed protocolFeeController);
    event ProtocolFeeUpdated(PoolId indexed id, uint24 protocolFee);
    event Swap(PoolId indexed id, address indexed sender, int128 amount0, int128 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick, uint24 fee);
    event Transfer(address caller, address indexed from, address indexed to, uint256 indexed id, uint256 amount);

    function allowance(address owner, address spender, uint256 id) external view returns (uint256 amount);
    function approve(address spender, uint256 id, uint256 amount) external returns (bool);
    function balanceOf(address owner, uint256 id) external view returns (uint256 balance);
    function burn(address from, uint256 id, uint256 amount) external;
    function clear(Currency currency, uint256 amount) external;
    function collectProtocolFees(address recipient, Currency currency, uint256 amount) external returns (uint256 amountCollected);
    function donate(PoolKey memory key, uint256 amount0, uint256 amount1, bytes memory hookData) external returns (BalanceDelta delta);
    function extsload(bytes32 slot) external view returns (bytes32);
    function extsload(bytes32 startSlot, uint256 nSlots) external view returns (bytes32[] memory);
    function extsload(bytes32[] memory slots) external view returns (bytes32[] memory);
    function exttload(bytes32[] memory slots) external view returns (bytes32[] memory);
    function exttload(bytes32 slot) external view returns (bytes32);
    function initialize(PoolKey memory key, uint160 sqrtPriceX96, bytes memory hookData) external returns (int24 tick);
    function isOperator(address owner, address operator) external view returns (bool isOperator);
    function mint(address to, uint256 id, uint256 amount) external;
    function modifyLiquidity(PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory hookData) external returns (BalanceDelta callerDelta, BalanceDelta feesAccrued);
    function owner() external view returns (address);
    function protocolFeeController() external view returns (address);
    function protocolFeesAccrued(Currency currency) external view returns (uint256 amount);
    function setOperator(address operator, bool approved) external returns (bool);
    function setProtocolFee(PoolKey memory key, uint24 newProtocolFee) external;
    function setProtocolFeeController(address controller) external;
    function settle() external payable returns (uint256);
    function settleFor(address recipient) external payable returns (uint256);
    function supportsInterface(bytes4 interfaceId) external view returns (bool);
    function swap(PoolKey memory key, IPoolManager.SwapParams memory params, bytes memory hookData) external returns (BalanceDelta swapDelta);
    function sync(Currency currency) external;
    function take(Currency currency, address to, uint256 amount) external;
    function transfer(address receiver, uint256 id, uint256 amount) external returns (bool);
    function transferFrom(address sender, address receiver, uint256 id, uint256 amount) external returns (bool);
    function transferOwnership(address newOwner) external;
    function unlock(bytes memory data) external returns (bytes memory result);
    function updateDynamicLPFee(PoolKey memory key, uint24 newDynamicLPFee) external;
}
```

...which was generated by the following JSON ABI:
```json
[
  {
    "type": "function",
    "name": "allowance",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "approve",
    "inputs": [
      {
        "name": "spender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "balanceOf",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "balance",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "burn",
    "inputs": [
      {
        "name": "from",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "clear",
    "inputs": [
      {
        "name": "currency",
        "type": "address",
        "internalType": "Currency"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "collectProtocolFees",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "currency",
        "type": "address",
        "internalType": "Currency"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "amountCollected",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "donate",
    "inputs": [
      {
        "name": "key",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
      },
      {
        "name": "amount0",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount1",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "hookData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "delta",
        "type": "int256",
        "internalType": "BalanceDelta"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "extsload",
    "inputs": [
      {
        "name": "slot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "extsload",
    "inputs": [
      {
        "name": "startSlot",
        "type": "bytes32",
        "internalType": "bytes32"
      },
      {
        "name": "nSlots",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "extsload",
    "inputs": [
      {
        "name": "slots",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "exttload",
    "inputs": [
      {
        "name": "slots",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32[]",
        "internalType": "bytes32[]"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "exttload",
    "inputs": [
      {
        "name": "slot",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bytes32",
        "internalType": "bytes32"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "initialize",
    "inputs": [
      {
        "name": "key",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
      },
      {
        "name": "sqrtPriceX96",
        "type": "uint160",
        "internalType": "uint160"
      },
      {
        "name": "hookData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "tick",
        "type": "int24",
        "internalType": "int24"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "isOperator",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "isOperator",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "mint",
    "inputs": [
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "modifyLiquidity",
    "inputs": [
      {
        "name": "key",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IPoolManager.ModifyLiquidityParams",
        "components": [
          {
            "name": "tickLower",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "tickUpper",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "liquidityDelta",
            "type": "int256",
            "internalType": "int256"
          },
          {
            "name": "salt",
            "type": "bytes32",
            "internalType": "bytes32"
          }
        ]
      },
      {
        "name": "hookData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "callerDelta",
        "type": "int256",
        "internalType": "BalanceDelta"
      },
      {
        "name": "feesAccrued",
        "type": "int256",
        "internalType": "BalanceDelta"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "owner",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "address"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "protocolFeeController",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "address",
        "internalType": "contract IProtocolFeeController"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "protocolFeesAccrued",
    "inputs": [
      {
        "name": "currency",
        "type": "address",
        "internalType": "Currency"
      }
    ],
    "outputs": [
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "setOperator",
    "inputs": [
      {
        "name": "operator",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "approved",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setProtocolFee",
    "inputs": [
      {
        "name": "key",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
      },
      {
        "name": "newProtocolFee",
        "type": "uint24",
        "internalType": "uint24"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "setProtocolFeeController",
    "inputs": [
      {
        "name": "controller",
        "type": "address",
        "internalType": "contract IProtocolFeeController"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "settle",
    "inputs": [],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "settleFor",
    "inputs": [
      {
        "name": "recipient",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "stateMutability": "payable"
  },
  {
    "type": "function",
    "name": "supportsInterface",
    "inputs": [
      {
        "name": "interfaceId",
        "type": "bytes4",
        "internalType": "bytes4"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "view"
  },
  {
    "type": "function",
    "name": "swap",
    "inputs": [
      {
        "name": "key",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
      },
      {
        "name": "params",
        "type": "tuple",
        "internalType": "struct IPoolManager.SwapParams",
        "components": [
          {
            "name": "zeroForOne",
            "type": "bool",
            "internalType": "bool"
          },
          {
            "name": "amountSpecified",
            "type": "int256",
            "internalType": "int256"
          },
          {
            "name": "sqrtPriceLimitX96",
            "type": "uint160",
            "internalType": "uint160"
          }
        ]
      },
      {
        "name": "hookData",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "swapDelta",
        "type": "int256",
        "internalType": "BalanceDelta"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "sync",
    "inputs": [
      {
        "name": "currency",
        "type": "address",
        "internalType": "Currency"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "take",
    "inputs": [
      {
        "name": "currency",
        "type": "address",
        "internalType": "Currency"
      },
      {
        "name": "to",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transfer",
    "inputs": [
      {
        "name": "receiver",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferFrom",
    "inputs": [
      {
        "name": "sender",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "receiver",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "internalType": "uint256"
      }
    ],
    "outputs": [
      {
        "name": "",
        "type": "bool",
        "internalType": "bool"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "transferOwnership",
    "inputs": [
      {
        "name": "newOwner",
        "type": "address",
        "internalType": "address"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "unlock",
    "inputs": [
      {
        "name": "data",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "outputs": [
      {
        "name": "result",
        "type": "bytes",
        "internalType": "bytes"
      }
    ],
    "stateMutability": "nonpayable"
  },
  {
    "type": "function",
    "name": "updateDynamicLPFee",
    "inputs": [
      {
        "name": "key",
        "type": "tuple",
        "internalType": "struct PoolKey",
        "components": [
          {
            "name": "currency0",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "currency1",
            "type": "address",
            "internalType": "Currency"
          },
          {
            "name": "fee",
            "type": "uint24",
            "internalType": "uint24"
          },
          {
            "name": "tickSpacing",
            "type": "int24",
            "internalType": "int24"
          },
          {
            "name": "hooks",
            "type": "address",
            "internalType": "contract IHooks"
          }
        ]
      },
      {
        "name": "newDynamicLPFee",
        "type": "uint24",
        "internalType": "uint24"
      }
    ],
    "outputs": [],
    "stateMutability": "nonpayable"
  },
  {
    "type": "event",
    "name": "Approval",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "spender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Donate",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "PoolId"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount0",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      },
      {
        "name": "amount1",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Initialize",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "PoolId"
      },
      {
        "name": "currency0",
        "type": "address",
        "indexed": true,
        "internalType": "Currency"
      },
      {
        "name": "currency1",
        "type": "address",
        "indexed": true,
        "internalType": "Currency"
      },
      {
        "name": "fee",
        "type": "uint24",
        "indexed": false,
        "internalType": "uint24"
      },
      {
        "name": "tickSpacing",
        "type": "int24",
        "indexed": false,
        "internalType": "int24"
      },
      {
        "name": "hooks",
        "type": "address",
        "indexed": false,
        "internalType": "contract IHooks"
      },
      {
        "name": "sqrtPriceX96",
        "type": "uint160",
        "indexed": false,
        "internalType": "uint160"
      },
      {
        "name": "tick",
        "type": "int24",
        "indexed": false,
        "internalType": "int24"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ModifyLiquidity",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "PoolId"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "tickLower",
        "type": "int24",
        "indexed": false,
        "internalType": "int24"
      },
      {
        "name": "tickUpper",
        "type": "int24",
        "indexed": false,
        "internalType": "int24"
      },
      {
        "name": "liquidityDelta",
        "type": "int256",
        "indexed": false,
        "internalType": "int256"
      },
      {
        "name": "salt",
        "type": "bytes32",
        "indexed": false,
        "internalType": "bytes32"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OperatorSet",
    "inputs": [
      {
        "name": "owner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "operator",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "approved",
        "type": "bool",
        "indexed": false,
        "internalType": "bool"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "OwnershipTransferred",
    "inputs": [
      {
        "name": "user",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "newOwner",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProtocolFeeControllerUpdated",
    "inputs": [
      {
        "name": "protocolFeeController",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "ProtocolFeeUpdated",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "PoolId"
      },
      {
        "name": "protocolFee",
        "type": "uint24",
        "indexed": false,
        "internalType": "uint24"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Swap",
    "inputs": [
      {
        "name": "id",
        "type": "bytes32",
        "indexed": true,
        "internalType": "PoolId"
      },
      {
        "name": "sender",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "amount0",
        "type": "int128",
        "indexed": false,
        "internalType": "int128"
      },
      {
        "name": "amount1",
        "type": "int128",
        "indexed": false,
        "internalType": "int128"
      },
      {
        "name": "sqrtPriceX96",
        "type": "uint160",
        "indexed": false,
        "internalType": "uint160"
      },
      {
        "name": "liquidity",
        "type": "uint128",
        "indexed": false,
        "internalType": "uint128"
      },
      {
        "name": "tick",
        "type": "int24",
        "indexed": false,
        "internalType": "int24"
      },
      {
        "name": "fee",
        "type": "uint24",
        "indexed": false,
        "internalType": "uint24"
      }
    ],
    "anonymous": false
  },
  {
    "type": "event",
    "name": "Transfer",
    "inputs": [
      {
        "name": "caller",
        "type": "address",
        "indexed": false,
        "internalType": "address"
      },
      {
        "name": "from",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "to",
        "type": "address",
        "indexed": true,
        "internalType": "address"
      },
      {
        "name": "id",
        "type": "uint256",
        "indexed": true,
        "internalType": "uint256"
      },
      {
        "name": "amount",
        "type": "uint256",
        "indexed": false,
        "internalType": "uint256"
      }
    ],
    "anonymous": false
  },
  {
    "type": "error",
    "name": "AlreadyUnlocked",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ContractUnlocked",
    "inputs": []
  },
  {
    "type": "error",
    "name": "CurrenciesOutOfOrderOrEqual",
    "inputs": [
      {
        "name": "currency0",
        "type": "address",
        "internalType": "address"
      },
      {
        "name": "currency1",
        "type": "address",
        "internalType": "address"
      }
    ]
  },
  {
    "type": "error",
    "name": "CurrencyNotSettled",
    "inputs": []
  },
  {
    "type": "error",
    "name": "DelegateCallNotAllowed",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidBips",
    "inputs": []
  },
  {
    "type": "error",
    "name": "InvalidCaller",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ManagerLocked",
    "inputs": []
  },
  {
    "type": "error",
    "name": "MustClearExactPositiveDelta",
    "inputs": []
  },
  {
    "type": "error",
    "name": "NonzeroNativeValue",
    "inputs": []
  },
  {
    "type": "error",
    "name": "PoolNotInitialized",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ProtocolFeeCannotBeFetched",
    "inputs": []
  },
  {
    "type": "error",
    "name": "ProtocolFeeTooLarge",
    "inputs": [
      {
        "name": "fee",
        "type": "uint24",
        "internalType": "uint24"
      }
    ]
  },
  {
    "type": "error",
    "name": "SwapAmountCannotBeZero",
    "inputs": []
  },
  {
    "type": "error",
    "name": "TickSpacingTooLarge",
    "inputs": [
      {
        "name": "tickSpacing",
        "type": "int24",
        "internalType": "int24"
      }
    ]
  },
  {
    "type": "error",
    "name": "TickSpacingTooSmall",
    "inputs": [
      {
        "name": "tickSpacing",
        "type": "int24",
        "internalType": "int24"
      }
    ]
  },
  {
    "type": "error",
    "name": "UnauthorizedDynamicLPFeeUpdate",
    "inputs": []
  }
]
```*/
#[allow(non_camel_case_types, non_snake_case, clippy::style)]
pub mod PoolManager {
    use super::*;
    use alloy::sol_types as alloy_sol_types;
    /// The creation / init bytecode of the contract.
    ///
    /// ```text
    ///0x60a0604052348015600e575f80fd5b505f80546001600160a01b031916339081178255604051909182917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0908290a35030608052608051616ae161006b5f395f6120660152616ae15ff3fe6080604052600436106101f4575f3560e01c80635a6bcfda11610117578063a5841194116100ac578063f135baaa1161007c578063f3cd914c11610062578063f3cd914c14610676578063f5298aca14610695578063fe99049a146106b4575f80fd5b8063f135baaa14610638578063f2fde38b14610657575f80fd5b8063a584119414610595578063b6363cf2146105b4578063dbd035ff146105ed578063f02de3b21461060c575f80fd5b80638161b874116100e75780638161b874146104dc5780638da5cb5b146104fb57806397e8cd4e1461054b5780639bf6645f14610576575f80fd5b80635a6bcfda14610438578063695c5bf51461046c5780637e87ce7d1461049e57806380f0b44c146104bd575f80fd5b80632d7713891161018d57806348c894911161015d57806348c894911461039257806352759651146103be578063558a7297146103dd578063598af9e7146103fc575f80fd5b80632d7713891461031557806335fd631a146103345780633dd45adb14610360578063426a849314610373575f80fd5b806311da60b4116101c857806311da60b4146102b0578063156e29f6146102b85780631e2eaeaf146102d7578063234266d7146102f6575f80fd5b8062fdd58e146101f857806301ffc9a714610241578063095bcdb6146102705780630b0d9c091461028f575b5f80fd5b348015610203575f80fd5b5061022e610212366004615a3c565b600460209081525f928352604080842090915290825290205481565b6040519081526020015b60405180910390f35b34801561024c575f80fd5b5061026061025b366004615a66565b6106d3565b6040519015158152602001610238565b34801561027b575f80fd5b5061026061028a366004615aa5565b61076b565b34801561029a575f80fd5b506102ae6102a9366004615ad7565b61083f565b005b61022e6108c9565b3480156102c3575f80fd5b506102ae6102d2366004615aa5565b610927565b3480156102e2575f80fd5b5061022e6102f1366004615b15565b6109ab565b348015610301575f80fd5b5061022e610310366004615ccf565b6109b5565b348015610320575f80fd5b506102ae61032f366004615d34565b610ad9565b34801561033f575f80fd5b5061035361034e366004615d4f565b610bcc565b6040516102389190615d6f565b61022e61036e366004615d34565b610c09565b34801561037e575f80fd5b5061026061038d366004615aa5565b610c67565b34801561039d575f80fd5b506103b16103ac366004615db1565b610cd8565b6040516102389190615df0565b3480156103c9575f80fd5b506102ae6103d8366004615e43565b610e2a565b3480156103e8575f80fd5b506102606103f7366004615e84565b610ecc565b348015610407575f80fd5b5061022e610416366004615ad7565b600560209081525f938452604080852082529284528284209052825290205481565b348015610443575f80fd5b50610457610452366004615eae565b610f66565b60408051928352602083019190915201610238565b348015610477575f80fd5b5061048b610486366004615f6f565b611165565b60405160029190910b8152602001610238565b3480156104a9575f80fd5b506102ae6104b8366004615e43565b611412565b3480156104c8575f80fd5b506102ae6104d7366004615a3c565b611503565b3480156104e7575f80fd5b5061022e6104f6366004615ad7565b6115c2565b348015610506575f80fd5b505f546105269073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff9091168152602001610238565b348015610556575f80fd5b5061022e610565366004615d34565b60016020525f908152604090205481565b348015610581575f80fd5b50610353610590366004615fb7565b6116ee565b3480156105a0575f80fd5b506102ae6105af366004615d34565b611727565b3480156105bf575f80fd5b506102606105ce366004616028565b600360209081525f928352604080842090915290825290205460ff1681565b3480156105f8575f80fd5b50610353610607366004615fb7565b6117cc565b348015610617575f80fd5b506002546105269073ffffffffffffffffffffffffffffffffffffffff1681565b348015610643575f80fd5b5061022e610652366004615b15565b611803565b348015610662575f80fd5b506102ae610671366004615d34565b61180d565b348015610681575f80fd5b5061022e61069036600461605f565b6118fc565b3480156106a0575f80fd5b506102ae6106af366004615aa5565b611aae565b3480156106bf575f80fd5b506102606106ce36600461611d565b611b32565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316148061076557507f0f632fb3000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316145b92915050565b335f90815260046020908152604080832085845290915281208054839190839061079690849061618d565b909155505073ffffffffffffffffffffffffffffffffffffffff84165f908152600460209081526040808320868452909152812080548492906107da9084906161a0565b9091555050604080513380825260208201859052859273ffffffffffffffffffffffffffffffffffffffff8816927f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac72885991015b60405180910390a45060015b9392505050565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c61088e5761088e7f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b6108a38361089b83611d26565b5f0333611d6b565b6108c473ffffffffffffffffffffffffffffffffffffffff84168383611dcb565b505050565b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610919576109197f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b61092233611ec6565b905090565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610976576109767f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b816109848161089b84611d26565b6109a58473ffffffffffffffffffffffffffffffffffffffff831684611faf565b50505050565b5f81545f5260205ff35b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610a0557610a057f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b610a0d61204e565b60a086205f818152600660205260409020610a27816120b6565b6080880151610a509073ffffffffffffffffffffffffffffffffffffffff1689898989896120fd565b610a5b8188886121e0565b9250610a688884336122da565b6040805188815260208101889052339184917f29ef05caaff9404b7cb6d1c0e9bbae9eaa7ab2541feba1a9c4248594c08156cb910160405180910390a36080880151610ace9073ffffffffffffffffffffffffffffffffffffffff168989898989612302565b505095945050505050565b5f5473ffffffffffffffffffffffffffffffffffffffff163314610b5e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f554e415554484f52495a4544000000000000000000000000000000000000000060448201526064015b60405180910390fd5b600280547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff83169081179091556040517fb4bd8ef53df690b9943d3318996006dbb82a25f54719d8c8035b516a2a5b8acc905f90a250565b6060604051808360051b6020835284602084015260408301925080830190505b85548352602083019250600186019550808310610bec5781810382f35b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610c5957610c597f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b61076582611ec6565b919050565b335f81815260056020908152604080832073ffffffffffffffffffffffffffffffffffffffff881680855290835281842087855290925280832085905551919285927fb3fd5071835887567a0671151121894ddccc2842f1d10bedad13e0d17cace9a79061082c9087815260200190565b60607fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c15610d2a57610d2a7f5090d6c600000000000000000000000000000000000000000000000000000000611d1e565b610d326123da565b6040517f91dd734600000000000000000000000000000000000000000000000000000000815233906391dd734690610d7090869086906004016161fa565b5f604051808303815f875af1158015610d8b573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610dd0919081019061620d565b90507f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5c15610e2257610e227f5212cba100000000000000000000000000000000000000000000000000000000611d1e565b610765612400565b604082015162ffffff1662800000141580610e755750816080015173ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614155b15610ea357610ea37f30d2164100000000000000000000000000000000000000000000000000000000611d1e565b610eb18162ffffff16612425565b60a082205f8181526006602052604090206108c49083612464565b335f81815260036020908152604080832073ffffffffffffffffffffffffffffffffffffffff871680855290835281842080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001687151590811790915591519182529293917fceb576d9f15e4e200fdb5096d64d5dfd667e16def20c1eefd14256d8e3faa267910160405180910390a350600192915050565b5f807fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610fb757610fb77f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b610fbf61204e565b60a086205f818152600660205260409020610fd9816120b6565b60808801516110019073ffffffffffffffffffffffffffffffffffffffff16898989896124bd565b5f6110756040518060c001604052803373ffffffffffffffffffffffffffffffffffffffff1681526020018a5f015160020b81526020018a6020015160020b81526020016110528b6040015161267e565b600f0b81526060808d015160020b60208301528b015160409091015283906126b3565b945090506110838185612af8565b945050503373ffffffffffffffffffffffffffffffffffffffff16817ff208f4912782fd25c7f114ca3723a2d5dd6f3bcc3ac8db5af63baa85f711d5ec885f015189602001518a604001518b606001516040516111019493929190600294850b81529290930b60208301526040820152606081019190915260800190565b60405180910390a360808701515f906111359073ffffffffffffffffffffffffffffffffffffffff16898987878b8b612b2b565b9094509050801561114f5761114f88828a608001516122da565b61115a8885336122da565b505094509492505050565b5f61116e61204e565b6060850151617fff60029190910b13156111b25760608501516111b2907fb70024f80000000000000000000000000000000000000000000000000000000090612d02565b600160020b856060015160020b12156111f55760608501516111f5907fe9e905880000000000000000000000000000000000000000000000000000000090612d02565b8451602086015173ffffffffffffffffffffffffffffffffffffffff90811691161061124d578451602086015161124d917f6e6c98300000000000000000000000000000000000000000000000000000000091612d11565b61127e8560400151866080015173ffffffffffffffffffffffffffffffffffffffff16612d5490919063ffffffff16565b6112b25760808501516112b2907fe65af6a00000000000000000000000000000000000000000000000000000000090612e22565b5f6112c5866040015162ffffff16612e44565b60808701519091506112f09073ffffffffffffffffffffffffffffffffffffffff1687878787612e69565b60a086205f6112fe88612f40565b5f83815260066020526040902090915061131a908883866130a8565b60808901519094506113469073ffffffffffffffffffffffffffffffffffffffff168989878a8a613183565b876020015173ffffffffffffffffffffffffffffffffffffffff16885f015173ffffffffffffffffffffffffffffffffffffffff16837fdd466e674ea557f56295e2d0218a125ea4b4f0f6f3307b95f85e6110838d64388b604001518c606001518d608001518d8b6040516113ff95949392919062ffffff959095168552600293840b602086015273ffffffffffffffffffffffffffffffffffffffff928316604086015291166060840152900b608082015260a00190565b60405180910390a4505050949350505050565b60025473ffffffffffffffffffffffffffffffffffffffff16331461145a5761145a7f48f5c3ed00000000000000000000000000000000000000000000000000000000611d1e565b6103e9610fff821610623e900062fff000831610166114a2576114a27fa7abe2f70000000000000000000000000000000000000000000000000000000062ffffff8316612e22565b60a082206114c4826114be835f90815260066020526040902090565b9061325c565b60405162ffffff8316815281907fe9c42593e71f84403b84352cd168d693e2c9fcd1fdbcc3feb21d92b43e6696f99060200160405180910390a2505050565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c611552576115527f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b335f90815273ffffffffffffffffffffffffffffffffffffffff8316602052604081205c9061158083611d26565b90508181600f0b146115b5576115b57fbda73abf00000000000000000000000000000000000000000000000000000000611d1e565b6109a584825f0333611d6b565b6002545f9073ffffffffffffffffffffffffffffffffffffffff16331461160c5761160c7f48f5c3ed00000000000000000000000000000000000000000000000000000000611d1e565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c1561165c5761165c7f3e5f4fd600000000000000000000000000000000000000000000000000000000611d1e565b8115611668578161168e565b73ffffffffffffffffffffffffffffffffffffffff83165f908152600160205260409020545b73ffffffffffffffffffffffffffffffffffffffff84165f908152600160205260408120805492935083929091906116c790849061618d565b90915550610838905073ffffffffffffffffffffffffffffffffffffffff84168583611dcb565b606060405180602082528360208301526040820191508360051b8201855b80355c84526020938401930181841061170c575b5081810382f35b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c611776576117767f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b73ffffffffffffffffffffffffffffffffffffffff811661179c576117996132b0565b50565b5f6117bc8273ffffffffffffffffffffffffffffffffffffffff166132d5565b90506117c88282613384565b5050565b606060405180602082528360208301526040820191508360051b8201855b80355484526020938401930181841015611720576117ea565b5f815c5f5260205ff35b5f5473ffffffffffffffffffffffffffffffffffffffff16331461188d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f554e415554484f52495a454400000000000000000000000000000000000000006044820152606401610b55565b5f80547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff83169081178255604051909133917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a350565b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c61194c5761194c7f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b61195461204e565b83602001515f03611988576119887fbe8b850700000000000000000000000000000000000000000000000000000000611d1e565b60a085205f8181526006602052604090206119a2816120b6565b60808701515f90819081906119d09073ffffffffffffffffffffffffffffffffffffffff168b8b8b8b6133e4565b809350819550829450505050611a4c84866040518060a001604052808681526020018e6060015160020b81526020018d5f0151151581526020018d6040015173ffffffffffffffffffffffffffffffffffffffff1681526020018562ffffff168152508c5f0151611a45578d6020015161358b565b8d5161358b565b60808b01519096505f9250611a7d915073ffffffffffffffffffffffffffffffffffffffff168a8a888b8b8861368c565b90955090508015611a9757611a9789828b608001516122da565b611aa28986336122da565b50505050949350505050565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c611afd57611afd7f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b81611b1181611b0b84611d26565b33611d6b565b6109a58473ffffffffffffffffffffffffffffffffffffffff831684613825565b5f3373ffffffffffffffffffffffffffffffffffffffff861614801590611b89575073ffffffffffffffffffffffffffffffffffffffff85165f90815260036020908152604080832033845290915290205460ff16155b15611c325773ffffffffffffffffffffffffffffffffffffffff85165f90815260056020908152604080832033845282528083208684529091529020547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8114611c3057611bf7838261618d565b73ffffffffffffffffffffffffffffffffffffffff87165f90815260056020908152604080832033845282528083208884529091529020555b505b73ffffffffffffffffffffffffffffffffffffffff85165f90815260046020908152604080832086845290915281208054849290611c7190849061618d565b909155505073ffffffffffffffffffffffffffffffffffffffff84165f90815260046020908152604080832086845290915281208054849290611cb59084906161a0565b90915550506040805133815260208101849052849173ffffffffffffffffffffffffffffffffffffffff80881692908916917f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac728859910160405180910390a45060015b949350505050565b805f5260045ffd5b5f6f800000000000000000000000000000008210611d6757611d677f93dafdf100000000000000000000000000000000000000000000000000000000611d1e565b5090565b81600f0b5f03611d7a57505050565b5f80611d9d73ffffffffffffffffffffffffffffffffffffffff86168486613936565b91509150805f03611db557611db061397c565b611dc4565b815f03611dc457611dc46139ca565b5050505050565b5f73ffffffffffffffffffffffffffffffffffffffff8416611e25575f805f8085875af1905080611e2057611e207f8549db590000000000000000000000000000000000000000000000000000000084613a18565b6109a5565b6040517fa9059cbb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8416600482015282602482015260205f6044835f895af13d15601f3d1160015f511416171691505f81525f60208201525f604082015250806109a5576109a57fb12c5f9c0000000000000000000000000000000000000000000000000000000085613a18565b5f7f27e098c505d44ec3574004bca052aabf76bd35004c182099d8c575fb238593b95c73ffffffffffffffffffffffffffffffffffffffff8116611f0c57349150611f96565b3415611f3b57611f3b7fb0ec849e00000000000000000000000000000000000000000000000000000000611d1e565b7f1e0745a7db1623981f0b2a5d4232364c00787266eb75ad546f190e6cebe9bd955c5f611f7d73ffffffffffffffffffffffffffffffffffffffff84166132d5565b9050611f89828261618d565b9350611f936132b0565b50505b611fa981611fa384611d26565b85611d6b565b50919050565b73ffffffffffffffffffffffffffffffffffffffff83165f90815260046020908152604080832085845290915281208054839290611fee9084906161a0565b90915550506040805133815260208101839052839173ffffffffffffffffffffffffffffffffffffffff8616915f917f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac72885991015b60405180910390a4505050565b3073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146120b4576120b47f0d89438e00000000000000000000000000000000000000000000000000000000611d1e565b565b805473ffffffffffffffffffffffffffffffffffffffff165f03611799576117997f486aa30700000000000000000000000000000000000000000000000000000000611d1e565b853373ffffffffffffffffffffffffffffffffffffffff8216146121d75760208716156121d7576121d5338787878787604051602401612142969594939291906162bf565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fb6a8b0fa0000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff891690613a4b565b505b50505050505050565b60038301545f906fffffffffffffffffffffffffffffffff16808203612229576122297fa74f97ab00000000000000000000000000000000000000000000000000000000611d1e565b61226061223585611d26565b5f0361224085611d26565b5f0360809190911b6fffffffffffffffffffffffffffffffff9091161790565b9150831561229a576001850180546fffffffffffffffffffffffffffffffff83167001000000000000000000000000000000008702040190555b82156122d2576002850180546fffffffffffffffffffffffffffffffff83167001000000000000000000000000000000008602040190555b509392505050565b82516122f0906122ea8460801d90565b83611d6b565b6108c483602001516122ea84600f0b90565b853373ffffffffffffffffffffffffffffffffffffffff8216146121d75760108716156121d7576121d5338787878787604051602401612347969594939291906162bf565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fe1b4af690000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff891690613a4b565b60017fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235d565b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235d565b620f424062ffffff82161115611799576117997f140021130000000000000000000000000000000000000000000000000000000062ffffff8316612e22565b61246d826120b6565b81547fffffff000000ffffffffffffffffffffffffffffffffffffffffffffffffffff167cffffff000000000000000000000000000000000000000000000000000060d083901b16175b90915550565b843373ffffffffffffffffffffffffffffffffffffffff821614612676575f84604001511380156124f15750610800861615155b156125ab576125a5338686868660405160240161251295949392919061638b565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f259982e50000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff881690613a4b565b50612676565b5f8460400151131580156125c25750610200861615155b15612676576121d733868686866040516024016125e395949392919061638b565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f21d0ee700000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff881690613a4b565b505050505050565b80600f81900b8114610c6257610c627f93dafdf100000000000000000000000000000000000000000000000000000000611d1e565b6060810151602082015160408301515f92839290916126d28282613b3e565b604080516080810182525f80825260208201819052918101829052606081019190915283600f0b5f146128a25761270b8884865f613c05565b6fffffffffffffffffffffffffffffffff166020830152151581526127338883866001613c05565b6fffffffffffffffffffffffffffffffff166060830152151560408201525f600f85900b126128675760808701515f9060020b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2761881810783139082900503620d89e891909105036001016fffffffffffffffffffffffffffffffff049050806fffffffffffffffffffffffffffffffff1682602001516fffffffffffffffffffffffffffffffff16111561280b5761280b7fb8e3c3850000000000000000000000000000000000000000000000000000000085612d02565b806fffffffffffffffffffffffffffffffff1682606001516fffffffffffffffffffffffffffffffff161115612865576128657fb8e3c3850000000000000000000000000000000000000000000000000000000084612d02565b505b8051156128835760808701516128839060058a01908590613cee565b8060400151156128a25760808701516128a29060058a01908490613cee565b5f806128af8a8686613d40565b8a5160a08c015160408051602681019290925260068083018a9052600383018b9052928252603a600c8301205f838301819052602080850182905293819052908152928f0190915281209294509092508061290c838a8787613df4565b9150915061294161291c83611d26565b61292583611d26565b6fffffffffffffffffffffffffffffffff1660809190911b1790565b995050505050505f84600f0b12156129aa5780511561297d57600283810b5f90815260048a016020526040812081815560018101829055909101555b8060400151156129aa57600282810b5f90815260048a016020526040812081815560018101829055909101555b5082600f0b5f14612aee5786545f806129c68360a01c60020b90565b73ffffffffffffffffffffffffffffffffffffffff8416915091508460020b8260020b1215612a2257612a1b612a15612a10612a0188613f25565b612a0a88613f25565b8a614222565b61267e565b60801b90565b9750612aea565b8360020b8260020b1215612ac557612a59612a43612a1083612a0a88613f25565b612925612a10612a5289613f25565b858b61425a565b60038b0154909850612a7d906fffffffffffffffffffffffffffffffff1687614286565b60038b0180547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166fffffffffffffffffffffffffffffffff92909216919091179055612aea565b612ae75f612925612a10612ad889613f25565b612ae189613f25565b8b61425a565b97505b5050505b5050509250929050565b5f608082811d9084901d01600f83810b9085900b01612b22612b198361267e565b6129258361267e565b95945050505050565b5f8073ffffffffffffffffffffffffffffffffffffffff89163303612b5457508490505f612cf6565b8591505f87604001511315612c3c57610400891615612c3757612c2833898989898989604051602401612b8d979695949392919061646d565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f9f063efc0000000000000000000000000000000000000000000000000000000017905260028b1615155b73ffffffffffffffffffffffffffffffffffffffff8c1691906142b6565b9050612c348282614310565b91505b612cf6565b610100891615612cf657612ce733898989898989604051602401612c66979695949392919061646d565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f6c2bbe7e0000000000000000000000000000000000000000000000000000000017905260018b161515612c0a565b9050612cf38282614310565b91505b97509795505050505050565b815f528060020b60045260245ffd5b60405183815273ffffffffffffffffffffffffffffffffffffffff8316600482015273ffffffffffffffffffffffffffffffffffffffff82166024820152604481fd5b5f60808316158015612d6857506008831615155b15612d7457505f610765565b60408316158015612d8757506004831615155b15612d9357505f610765565b6104008316158015612da757506002831615155b15612db357505f610765565b6101008316158015612dc757506001831615155b15612dd357505f610765565b73ffffffffffffffffffffffffffffffffffffffff831615612e1157613fff8316151580612e0c57506280000062ffffff831614610838565b610838565b5062ffffff16628000001415919050565b815f5273ffffffffffffffffffffffffffffffffffffffff811660045260245ffd5b5f6280000062ffffff831603612e5b57505f919050565b611d678262ffffff16612425565b843373ffffffffffffffffffffffffffffffffffffffff82161461267657612000861615612676576121d73386868686604051602401612ead95949392919061655d565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f3440d8200000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff881690613a4b565b6002545f9073ffffffffffffffffffffffffffffffffffffffff1615610c62575f612f6c456064614331565b9050805a1015612f9f57612f9f7f1ee4970200000000000000000000000000000000000000000000000000000000611d1e565b60025460405173ffffffffffffffffffffffffffffffffffffffff909116905f90612fce90869060240161662c565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152919052602080820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f553bfc370000000000000000000000000000000000000000000000000000000017815282519293505f9283929183919082888af15f513d6020149091169250905081801561307457508062ffffff1681145b801561309157506103e9610fff821610623e900062fff000831610165b61309b575f61309d565b805b979650505050505050565b83545f9073ffffffffffffffffffffffffffffffffffffffff16156130f0576130f07f7983c05100000000000000000000000000000000000000000000000000000000611d1e565b6130f984614385565b90507cffffff000000000000000000000000000000000000000000000000000060d083901b1676ffffff000000000000000000000000000000000000000060a083901b1673ffffffffffffffffffffffffffffffffffffffff86161760b885901b79ffffff0000000000000000000000000000000000000000000000161717909455509192915050565b853373ffffffffffffffffffffffffffffffffffffffff8216146121d7576110008716156121d7576121d53387878787876040516024016131c9969594939291906166af565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fa910f80f0000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff891690613a4b565b613265826120b6565b81547fffffffffffff000000ffffffffffffffffffffffffffffffffffffffffffffff1679ffffff000000000000000000000000000000000000000000000060b883901b16176124b7565b5f7f27e098c505d44ec3574004bca052aabf76bd35004c182099d8c575fb238593b95d565b5f73ffffffffffffffffffffffffffffffffffffffff82166132f8575047919050565b6040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff8316906370a0823190602401602060405180830381865afa158015613360573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107659190616788565b73ffffffffffffffffffffffffffffffffffffffff82167f27e098c505d44ec3574004bca052aabf76bd35004c182099d8c575fb238593b95d807f1e0745a7db1623981f0b2a5d4232364c00787266eb75ad546f190e6cebe9bd955d5050565b60208301515f8073ffffffffffffffffffffffffffffffffffffffff88163303613410575f9150613580565b6080881615613580575f6134b289338a8a8a8a60405160240161343795949392919061679f565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f575e24b400000000000000000000000000000000000000000000000000000000179052613a4b565b905080516060146134e6576134e67f1e048e1d00000000000000000000000000000000000000000000000000000000611d1e565b604088015162ffffff16628000000361350157606081015191505b600889161561357e57604081015192505f61351c8460801d90565b905080600f0b5f1461357c575f8512613539600f83900b87616887565b955080613548575f861261354c565b5f86135b1561357a5761357a7ffa0b71d600000000000000000000000000000000000000000000000000000000611d1e565b505b505b505b955095509592505050565b5f8080808061359a89886146a9565b93509350935093505f8311156135d55773ffffffffffffffffffffffffffffffffffffffff86165f9081526001602052604090208054840190555b33887f40e9cecb9f5f1f1c5b9c97dec2917b7ee92e57ba5563708daca94dd84ad7112f6136028760801d90565b61360c88600f0b90565b85516040808801516020808a01518351600f97880b81529590960b9085015273ffffffffffffffffffffffffffffffffffffffff909216908301526fffffffffffffffffffffffffffffffff16606082015260029190910b608082015262ffffff861660a082015260c00160405180910390a35091979650505050505050565b5f8073ffffffffffffffffffffffffffffffffffffffff891633036136b557508490505f612cf6565b5f6136c08460801d90565b90505f6136cd85600f0b90565b905060408b16156137a057613793612a10338c8c8c8c8c6040516024016136f9969594939291906168ae565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fb47b2fb10000000000000000000000000000000000000000000000000000000017905260048e16151573ffffffffffffffffffffffffffffffffffffffff8f1691906142b6565b61379d908261699d565b90505b5f81600f0b5f1415806137b6575082600f0b5f14155b1561381357895160208b01515f13901515146137ea576fffffffffffffffffffffffffffffffff8316608083901b17613804565b6fffffffffffffffffffffffffffffffff8216608084901b175b90506138108982614310565b98505b979b979a509698505050505050505050565b3373ffffffffffffffffffffffffffffffffffffffff8416811480159061387e575073ffffffffffffffffffffffffffffffffffffffff8085165f9081526003602090815260408083209385168352929052205460ff16155b1561392b5773ffffffffffffffffffffffffffffffffffffffff8085165f9081526005602090815260408083209385168352928152828220868352905220547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8114613929576138ee838261618d565b73ffffffffffffffffffffffffffffffffffffffff8087165f9081526005602090815260408083209387168352928152828220888352905220555b505b6109a5848484614f3b565b73ffffffffffffffffffffffffffffffffffffffff8281165f90815290841660205260408120805c919061396e600f85900b84616887565b915081815d50935093915050565b7f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5c600181039050807f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5d50565b7f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5c600181019050807f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5d50565b3d60405183815282600482015260406024820152816044820152815f606483013e602080601f8401040260640191508181fd5b60605f805f8451602086015f885af1905080613a8b57613a8b7f319d54c30000000000000000000000000000000000000000000000000000000085613a18565b6040519150601f19603f3d011682016040523d82523d5f602084013e602082511080613b09575060208301517fffffffff0000000000000000000000000000000000000000000000000000000016613ae4836020015190565b7fffffffff000000000000000000000000000000000000000000000000000000001614155b15613b3757613b377f1e048e1d00000000000000000000000000000000000000000000000000000000611d1e565b5092915050565b8060020b8260020b12613b7657613b767fc4433ed5000000000000000000000000000000000000000000000000000000008383614fd1565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff27618600283900b1215613bcc57613bcc7fd5e2f7ab0000000000000000000000000000000000000000000000000000000083612d02565b620d89e8600282900b13156117c8576117c87f1ad777f80000000000000000000000000000000000000000000000000000000082612d02565b600283900b5f908152600485016020526040812080548291906fffffffffffffffffffffffffffffffff8116907001000000000000000000000000000000009004600f0b613c538288614286565b6fffffffffffffffffffffffffffffffff808216159084168015919091141596509094505f03613ca657885460a01c60020b60020b8860020b13613ca6576001808a0154908401556002808a0154908401555b5f86613cbb57613cb6888361699d565b613cc5565b613cc588836169eb565b90508060801b6fffffffffffffffffffffffffffffffff86161784555050505094509492505050565b600291820b910b80820715613d1b5760405163d4d8f3e681528260208201528160408201526044601c8201fd5b80820591508160081d5f528260205260405f20600160ff84161b815418815550505050565b600282810b5f81815260048601602052604080822085850b83529082208754929485949293919260a09290921c900b90811215613d96578160010154836001015403945081600201548360020154039350613de9565b8560020b8160020b12613dc2578260010154826001015403945082600201548260020154039350613de9565b81600101548360010154896001015403039450816002015483600201548960020154030393505b505050935093915050565b83545f9081906fffffffffffffffffffffffffffffffff16600f86900b8203613e5e57806fffffffffffffffffffffffffffffffff165f03613e5957613e597faefeb92400000000000000000000000000000000000000000000000000000000611d1e565b613ea5565b613e688187614286565b87547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166fffffffffffffffffffffffffffffffff919091161787555b613ed987600101548603826fffffffffffffffffffffffffffffffff16700100000000000000000000000000000000614fee565b9250613f0f87600201548503826fffffffffffffffffffffffffffffffff16700100000000000000000000000000000000614fee565b6001880195909555505060029094015591929050565b60020b5f60ff82901d80830118620d89e8811115613f6757613f677f8b86327a0000000000000000000000000000000000000000000000000000000084612d02565b7001fffcb933bd6fad37aa2d162d1a5940016001821602700100000000000000000000000000000000186002821615613fb0576ffff97272373d413259a46990580e213a0260801c5b6004821615613fcf576ffff2e50f5f656932ef12357cf3c7fdcc0260801c5b6008821615613fee576fffe5caca7e10e4e61c3624eaa0941cd00260801c5b601082161561400d576fffcb9843d60f6159c9db58835c9266440260801c5b602082161561402c576fff973b41fa98c081472e6896dfb254c00260801c5b604082161561404b576fff2ea16466c96a3843ec78b326b528610260801c5b608082161561406a576ffe5dee046a99a2a811c461f1969c30530260801c5b61010082161561408a576ffcbe86c7900a88aedcffc83b479aa3a40260801c5b6102008216156140aa576ff987a7253ac413176f2b074cf7815e540260801c5b6104008216156140ca576ff3392b0822b70005940c7a398e4b70f30260801c5b6108008216156140ea576fe7159475a2c29b7443b29c7fa6e889d90260801c5b61100082161561410a576fd097f3bdfd2022b8845ad8f792aa58250260801c5b61200082161561412a576fa9f746462d870fdf8a65dc1f90e061e50260801c5b61400082161561414a576f70d869a156d2a1b890bb3df62baf32f70260801c5b61800082161561416a576f31be135f97d08fd981231505542fcfa60260801c5b6201000082161561418b576f09aa508b5b7a84e1c677de54f3e99bc90260801c5b620200008216156141ab576e5d6af8dedb81196699c329225ee6040260801c5b620400008216156141ca576d2216e584f5fa1ea926041bedfe980260801c5b620800008216156141e7576b048a170391f7dc42444e8fa20260801c5b5f841315614212577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff045b63ffffffff0160201c9392505050565b5f8082600f0b126142495761424261423d85858560016150a9565b6151db565b5f03611d16565b611d1661423d8585855f035f6150a9565b5f8082600f0b126142755761424261423d858585600161520d565b611d1661423d8585855f035f61520d565b6fffffffffffffffffffffffffffffffff8216600f82900b01608081901c15610765576393dafdf15f526004601cfd5b5f806142c28585613a4b565b9050826142d2575f915050610838565b8051604014614304576143047f1e048e1d00000000000000000000000000000000000000000000000000000000611d1e565b60400151949350505050565b5f608082811d9084901d03600f83810b9085900b03612b22612b198361267e565b5f61271082111561436e576040517fdeaa01e600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61271061437b8385616a39565b6108389190616a7d565b5f73fffd8963efd1fc6a506488495d951d51639616827ffffffffffffffffffffffffffffffffffffffffffffffffffffffffefffd895d830173ffffffffffffffffffffffffffffffffffffffff161115614404576144047f614875240000000000000000000000000000000000000000000000000000000083612e22565b77ffffffffffffffffffffffffffffffffffffffff00000000602083901b16805f61442e82615278565b60ff1690506080811061444957607f810383901c9150614453565b80607f0383901b91505b908002607f81811c60ff83811c9190911c800280831c81831c1c800280841c81841c1c800280851c81851c1c800280861c81861c1c800280871c81871c1c800280881c81881c1c800280891c81891c1c8002808a1c818a1c1c8002808b1c818b1c1c8002808c1c818c1c1c8002808d1c818d1c1c8002808e1c9c81901c9c909c1c80029c8d901c9e9d7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff808f0160401b60c09190911c678000000000000000161760c19b909b1c674000000000000000169a909a1760c29990991c672000000000000000169890981760c39790971c671000000000000000169690961760c49590951c670800000000000000169490941760c59390931c670400000000000000169290921760c69190911c670200000000000000161760c79190911c670100000000000000161760c89190911c6680000000000000161760c99190911c6640000000000000161760ca9190911c6620000000000000161760cb9190911c6610000000000000161760cc9190911c6608000000000000161760cd9190911c66040000000000001617693627a301d71055774c8581027ffffffffffffffffffffffffffffffffffd709b7e5480fba5a50fed5e62ffc5568101608090811d906fdb2df09e81959a81455e260799a0632f8301901d600281810b9083900b1461469a578873ffffffffffffffffffffffffffffffffffffffff1661467282613f25565b73ffffffffffffffffffffffffffffffffffffffff161115614694578161469c565b8061469c565b815b9998505050505050505050565b604080516060810182525f8082526020820181905291810182905281908190855460408601515f816146e357610fff60c484901c166146ed565b610fff60b884901c165b885173ffffffffffffffffffffffffffffffffffffffff8516865261ffff9190911691505f60a085901c60020b60020b602087015260038b01546fffffffffffffffffffffffffffffffff16604087015260808a01515f90624000001661475d5760d086901c62ffffff1661476f565b61476f8b6080015162ffffff1661530c565b9050831561479d57620f4240610fff851662ffffff831681810283810615159390049290920191010361479f565b805b975050620f42408762ffffff16106147e35789515f12156147e3576147e37f9620624600000000000000000000000000000000000000000000000000000000611d1e565b89515f036147fb575f80985098505050505050614f32565b83156148de5760608a015173ffffffffffffffffffffffffffffffffffffffff86811691161061486d5761486d73ffffffffffffffffffffffffffffffffffffffff86165b60608c01517f7c9c6e8f000000000000000000000000000000000000000000000000000000009190612d11565b6401000276a373ffffffffffffffffffffffffffffffffffffffff168a6060015173ffffffffffffffffffffffffffffffffffffffff16116148d95760608a01516148d9907f9e4d7cc70000000000000000000000000000000000000000000000000000000090612e22565b61499c565b60608a015173ffffffffffffffffffffffffffffffffffffffff8681169116116149215761492173ffffffffffffffffffffffffffffffffffffffff8616614840565b73fffd8963efd1fc6a506488495d951d5263988d2673ffffffffffffffffffffffffffffffffffffffff168a6060015173ffffffffffffffffffffffffffffffffffffffff161061499c5760608a015161499c907f9e4d7cc70000000000000000000000000000000000000000000000000000000090612e22565b60408051610100810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810191909152846149eb578b600201546149f1565b8b600101545b60e08201525b821580614a3657508a6060015173ffffffffffffffffffffffffffffffffffffffff16875f015173ffffffffffffffffffffffffffffffffffffffff16145b614dbe57865173ffffffffffffffffffffffffffffffffffffffff168152602080880151908c0151614a6d9160058f01918861531b565b1515604083015260020b602082018190527ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2761812614acb577ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2761860208201525b620d89e860020b816020015160020b12614ae957620d89e860208201525b614af68160200151613f25565b73ffffffffffffffffffffffffffffffffffffffff90811660608381018290528951908e0151614b40939192911680821891811160018a161891909102188960400151868c615446565b60c085015260a0840152608083015273ffffffffffffffffffffffffffffffffffffffff1687528a515f1215614ba957614b7d8160a001516151db565b83039250614b988160c00151826080015161423d91906161a0565b614ba29083616ab5565b9150614bda565b614bbc8160c001518260800151016151db565b83019250614bcd8160a001516151db565b614bd79083616887565b91505b8315614c16575f620f4240858360c001518460800151010281614bff57614bff616a50565b60c084018051929091049182900390529990990198505b60408701516fffffffffffffffffffffffffffffffff1615614c7557614c698160c0015170010000000000000000000000000000000089604001516fffffffffffffffffffffffffffffffff1691020490565b60e08201805190910190525b806060015173ffffffffffffffffffffffffffffffffffffffff16875f015173ffffffffffffffffffffffffffffffffffffffff1603614d8b57806040015115614d66575f8086614ccf578d600101548360e00151614cda565b8260e001518e600201545b915091505f614d328f85602001518585600292830b5f908152600490940160205260409093206001810180549092039091559081018054909203909155547001000000000000000000000000000000009004600f0b90565b90508715614d3d575f035b614d4b8a6040015182614286565b6fffffffffffffffffffffffffffffffff1660408b01525050505b84614d75578060200151614d7e565b60018160200151035b60020b60208801526149f7565b8051875173ffffffffffffffffffffffffffffffffffffffff908116911614614db9578651614d7e90614385565b6149f7565b86516020880151614e539190614e1590899060a01b76ffffff0000000000000000000000000000000000000000167fffffffffffffffffff000000ffffffffffffffffffffffffffffffffffffffff919091161790565b7fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff9091161790565b8c55604087015160038d01546fffffffffffffffffffffffffffffffff908116911614614ec257604087015160038d0180547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166fffffffffffffffffffffffffffffffff9092169190911790555b84614ed65760e081015160028d0155614ee1565b60e081015160018d01555b8a515f1385151514614f0e57614f07614ef98361267e565b612925858e5f01510361267e565b9950614f2b565b614f28614f1f848d5f01510361267e565b6129258461267e565b99505b5050505050505b92959194509250565b73ffffffffffffffffffffffffffffffffffffffff83165f90815260046020908152604080832085845290915281208054839290614f7a90849061618d565b9091555050604080513381526020810183905283915f9173ffffffffffffffffffffffffffffffffffffffff8716917f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac7288599101612041565b6040518381528260020b60048201528160020b6024820152604481fd5b5f838302817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8587098281108382030391505080841161502c575f80fd5b805f0361503e57508290049050610838565b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f8373ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff1611156150e2579293925b73ffffffffffffffffffffffffffffffffffffffff85166151095762bfc9215f526004601cfd5b7bffffffffffffffffffffffffffffffff000000000000000000000000606084901b1673ffffffffffffffffffffffffffffffffffffffff8686031683615195578673ffffffffffffffffffffffffffffffffffffffff1661518283838973ffffffffffffffffffffffffffffffffffffffff16614fee565b8161518f5761518f616a50565b0461309d565b61309d6151b983838973ffffffffffffffffffffffffffffffffffffffff166155b6565b8873ffffffffffffffffffffffffffffffffffffffff16808204910615150190565b805f811215610c6257610c627f93dafdf100000000000000000000000000000000000000000000000000000000611d1e565b5f73ffffffffffffffffffffffffffffffffffffffff8481169086160360ff81901d908101186c010000000000000000000000006fffffffffffffffffffffffffffffffff851661525f818484614fee565b9350845f83858409111684019350505050949350505050565b5f808211615284575f80fd5b507f0706060506020500060203020504000106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be6fffffffffffffffffffffffffffffffff841160071b84811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b62bfffff8116610c6281612425565b5f80600284810b9086900b81810783139190050383156153b957600281900b60081d600181900b5f908152602089905260409020547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60ff808516908190039190911c91821680151595509091908561539b57888360ff168603026153ae565b886153a582615278565b840360ff168603025b96505050505061543c565b6001908101600281900b60081d80830b5f90815260208a905260409020547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60ff841694851b01199081168015159550929391928561542257888360ff0360ff16860102615435565b888361542d836155e6565b0360ff168601025b9650505050505b5094509492505050565b5f80808062ffffff851673ffffffffffffffffffffffffffffffffffffffff808a16908b1610158288128015615529575f61548c8a5f0385620f424003620f4240614fee565b9050826154a5576154a08d8d8d600161520d565b6154b2565b6154b28c8e8d60016150a9565b96508681106154e6578b9750620f424084146154dd576154d8878586620f4240036155b6565b6154df565b865b94506154ff565b8096506154f58d8c8386615680565b9750868a5f030394505b82615515576155108d898d5f6150a9565b615521565b615521888e8d5f61520d565b9550506155a7565b8161553f5761553a8c8c8c5f6150a9565b61554b565b61554b8b8d8c5f61520d565b945084891061555c578a965061556e565b88945061556b8c8b87856156e4565b96505b81615585576155808c888c600161520d565b615592565b615592878d8c60016150a9565b95506155a4868485620f4240036155b6565b93505b50505095509550955095915050565b5f6155c2848484614fee565b905081806155d2576155d2616a50565b838509156108385760010180610838575f80fd5b5f8082116155f2575f80fd5b507e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b15031604055f8290039091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b5f6fffffffffffffffffffffffffffffffff84161573ffffffffffffffffffffffffffffffffffffffff86161517156156c057634f2461b85f526004601cfd5b816156d7576156d2858585600161573d565b612b22565b612b22858585600161589f565b5f6fffffffffffffffffffffffffffffffff84161573ffffffffffffffffffffffffffffffffffffffff861615171561572457634f2461b85f526004601cfd5b81615735576156d28585855f61589f565b612b228585855f5b5f81156157e2575f73ffffffffffffffffffffffffffffffffffffffff8411156157905761578b846c01000000000000000000000000876fffffffffffffffffffffffffffffffff16614fee565b6157b0565b6157b06fffffffffffffffffffffffffffffffff8616606086901b616a7d565b90506157da6157d58273ffffffffffffffffffffffffffffffffffffffff89166161a0565b6159d4565b915050611d16565b5f73ffffffffffffffffffffffffffffffffffffffff84111561582e57615829846c01000000000000000000000000876fffffffffffffffffffffffffffffffff166155b6565b615854565b615854606085901b6fffffffffffffffffffffffffffffffff8716808204910615150190565b90508073ffffffffffffffffffffffffffffffffffffffff87161161588057634323a5555f526004601cfd5b73ffffffffffffffffffffffffffffffffffffffff8616039050611d16565b5f825f036158ae575083611d16565b7bffffffffffffffffffffffffffffffff000000000000000000000000606085901b1682156159795773ffffffffffffffffffffffffffffffffffffffff86168481029085828161590157615901616a50565b040361593e5781810182811061593c57615932838973ffffffffffffffffffffffffffffffffffffffff16836155b6565b9350505050611d16565b505b506157da818561596473ffffffffffffffffffffffffffffffffffffffff8a1683616a7d565b61596e91906161a0565b808204910615150190565b73ffffffffffffffffffffffffffffffffffffffff86168481029085820414818311166159ad5763f5c787f15f526004601cfd5b8082036159326157d58473ffffffffffffffffffffffffffffffffffffffff8b16846155b6565b8073ffffffffffffffffffffffffffffffffffffffff81168114610c6257610c627f93dafdf100000000000000000000000000000000000000000000000000000000611d1e565b73ffffffffffffffffffffffffffffffffffffffff81168114611799575f80fd5b5f8060408385031215615a4d575f80fd5b8235615a5881615a1b565b946020939093013593505050565b5f60208284031215615a76575f80fd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114610838575f80fd5b5f805f60608486031215615ab7575f80fd5b8335615ac281615a1b565b95602085013595506040909401359392505050565b5f805f60608486031215615ae9575f80fd5b8335615af481615a1b565b92506020840135615b0481615a1b565b929592945050506040919091013590565b5f60208284031215615b25575f80fd5b5035919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040516080810167ffffffffffffffff81118282101715615b7c57615b7c615b2c565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715615bc957615bc9615b2c565b604052919050565b803562ffffff81168114610c62575f80fd5b8035600281900b8114610c62575f80fd5b5f60a08284031215615c04575f80fd5b60405160a0810167ffffffffffffffff81118282101715615c2757615c27615b2c565b6040529050808235615c3881615a1b565b81526020830135615c4881615a1b565b6020820152615c5960408401615bd1565b6040820152615c6a60608401615be3565b60608201526080830135615c7d81615a1b565b6080919091015292915050565b5f8083601f840112615c9a575f80fd5b50813567ffffffffffffffff811115615cb1575f80fd5b602083019150836020828501011115615cc8575f80fd5b9250929050565b5f805f805f6101008688031215615ce4575f80fd5b615cee8787615bf4565b945060a0860135935060c0860135925060e086013567ffffffffffffffff811115615d17575f80fd5b615d2388828901615c8a565b969995985093965092949392505050565b5f60208284031215615d44575f80fd5b813561083881615a1b565b5f8060408385031215615d60575f80fd5b50508035926020909101359150565b602080825282518282018190525f918401906040840190835b81811015615da6578351835260209384019390920191600101615d88565b509095945050505050565b5f8060208385031215615dc2575f80fd5b823567ffffffffffffffff811115615dd8575f80fd5b615de485828601615c8a565b90969095509350505050565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011684010191505092915050565b5f8060c08385031215615e54575f80fd5b615e5e8484615bf4565b9150615e6c60a08401615bd1565b90509250929050565b80358015158114610c62575f80fd5b5f8060408385031215615e95575f80fd5b8235615ea081615a1b565b9150615e6c60208401615e75565b5f805f80848603610140811215615ec3575f80fd5b615ecd8787615bf4565b945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6082011215615efe575f80fd5b50615f07615b59565b615f1360a08701615be3565b8152615f2160c08701615be3565b602082015260e086013560408201526101008601356060820152925061012085013567ffffffffffffffff811115615f57575f80fd5b615f6387828801615c8a565b95989497509550505050565b5f805f8060e08587031215615f82575f80fd5b615f8c8686615bf4565b935060a0850135615f9c81615a1b565b925060c085013567ffffffffffffffff811115615f57575f80fd5b5f8060208385031215615fc8575f80fd5b823567ffffffffffffffff811115615fde575f80fd5b8301601f81018513615fee575f80fd5b803567ffffffffffffffff811115616004575f80fd5b8560208260051b8401011115616018575f80fd5b6020919091019590945092505050565b5f8060408385031215616039575f80fd5b823561604481615a1b565b9150602083013561605481615a1b565b809150509250929050565b5f805f80848603610120811215616074575f80fd5b61607e8787615bf4565b945060607fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60820112156160af575f80fd5b506040516060810167ffffffffffffffff811182821017156160d3576160d3615b2c565b6040526160e260a08701615e75565b815260c0860135602082015260e08601356160fc81615a1b565b6040820152925061010085013567ffffffffffffffff811115615f57575f80fd5b5f805f8060808587031215616130575f80fd5b843561613b81615a1b565b9350602085013561614b81615a1b565b93969395505050506040820135916060013590565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b8181038181111561076557610765616160565b8082018082111561076557610765616160565b81835281816020850137505f602082840101525f60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116840101905092915050565b602081525f611d166020830184866161b3565b5f6020828403121561621d575f80fd5b815167ffffffffffffffff811115616233575f80fd5b8201601f81018413616243575f80fd5b805167ffffffffffffffff81111561625d5761625d615b2c565b61628e60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601615b82565b8181528560208385010111156162a2575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b73ffffffffffffffffffffffffffffffffffffffff8716815261635a602082018773ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8460c08201528360e08201526101206101008201525f61637f610120830184866161b3565b98975050505050505050565b73ffffffffffffffffffffffffffffffffffffffff86168152616426602082018673ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8351600290810b60c08301526020850151900b60e0820152604084015161010082015260608401516101208201526101606101408201525f61309d610160830184866161b3565b73ffffffffffffffffffffffffffffffffffffffff88168152616508602082018873ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8551600290810b60c08301526020870151900b60e08201526040860151610100820152606086015161012082015284610140820152836101608201526101a06101808201525f61469c6101a0830184866161b3565b73ffffffffffffffffffffffffffffffffffffffff861681526165f8602082018673ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff841660c082015261010060e08201525f61309d610100830184866161b3565b60a08101610765828473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff8716815261674a602082018773ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff851660c08201528360020b60e08201526101206101008201525f61637f610120830184866161b3565b5f60208284031215616798575f80fd5b5051919050565b73ffffffffffffffffffffffffffffffffffffffff8616815261683a602082018673ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8351151560c0820152602084015160e0820152604084015173ffffffffffffffffffffffffffffffffffffffff166101008201526101406101208201525f61309d610140830184866161b3565b8082018281125f8312801582168215821617156168a6576168a6616160565b505092915050565b73ffffffffffffffffffffffffffffffffffffffff87168152616949602082018773ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8451151560c0820152602085015160e0820152604085015173ffffffffffffffffffffffffffffffffffffffff16610100820152836101208201526101606101408201525f61637f610160830184866161b3565b600f81810b9083900b016f7fffffffffffffffffffffffffffffff81137fffffffffffffffffffffffffffffffff800000000000000000000000000000008212171561076557610765616160565b600f82810b9082900b037fffffffffffffffffffffffffffffffff8000000000000000000000000000000081126f7fffffffffffffffffffffffffffffff8213171561076557610765616160565b808202811582820484141761076557610765616160565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f82616ab0577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b8181035f831280158383131683831282161715613b3757613b3761616056fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\xA0`@R4\x80\x15`\x0EW_\x80\xFD[P_\x80T`\x01`\x01`\xA0\x1B\x03\x19\x163\x90\x81\x17\x82U`@Q\x90\x91\x82\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x90\x82\x90\xA3P0`\x80R`\x80Qaj\xE1a\0k_9_a f\x01Raj\xE1_\xF3\xFE`\x80`@R`\x046\x10a\x01\xF4W_5`\xE0\x1C\x80cZk\xCF\xDA\x11a\x01\x17W\x80c\xA5\x84\x11\x94\x11a\0\xACW\x80c\xF15\xBA\xAA\x11a\0|W\x80c\xF3\xCD\x91L\x11a\0bW\x80c\xF3\xCD\x91L\x14a\x06vW\x80c\xF5)\x8A\xCA\x14a\x06\x95W\x80c\xFE\x99\x04\x9A\x14a\x06\xB4W_\x80\xFD[\x80c\xF15\xBA\xAA\x14a\x068W\x80c\xF2\xFD\xE3\x8B\x14a\x06WW_\x80\xFD[\x80c\xA5\x84\x11\x94\x14a\x05\x95W\x80c\xB66<\xF2\x14a\x05\xB4W\x80c\xDB\xD05\xFF\x14a\x05\xEDW\x80c\xF0-\xE3\xB2\x14a\x06\x0CW_\x80\xFD[\x80c\x81a\xB8t\x11a\0\xE7W\x80c\x81a\xB8t\x14a\x04\xDCW\x80c\x8D\xA5\xCB[\x14a\x04\xFBW\x80c\x97\xE8\xCDN\x14a\x05KW\x80c\x9B\xF6d_\x14a\x05vW_\x80\xFD[\x80cZk\xCF\xDA\x14a\x048W\x80ci\\[\xF5\x14a\x04lW\x80c~\x87\xCE}\x14a\x04\x9EW\x80c\x80\xF0\xB4L\x14a\x04\xBDW_\x80\xFD[\x80c-w\x13\x89\x11a\x01\x8DW\x80cH\xC8\x94\x91\x11a\x01]W\x80cH\xC8\x94\x91\x14a\x03\x92W\x80cRu\x96Q\x14a\x03\xBEW\x80cU\x8Ar\x97\x14a\x03\xDDW\x80cY\x8A\xF9\xE7\x14a\x03\xFCW_\x80\xFD[\x80c-w\x13\x89\x14a\x03\x15W\x80c5\xFDc\x1A\x14a\x034W\x80c=\xD4Z\xDB\x14a\x03`W\x80cBj\x84\x93\x14a\x03sW_\x80\xFD[\x80c\x11\xDA`\xB4\x11a\x01\xC8W\x80c\x11\xDA`\xB4\x14a\x02\xB0W\x80c\x15n)\xF6\x14a\x02\xB8W\x80c\x1E.\xAE\xAF\x14a\x02\xD7W\x80c#Bf\xD7\x14a\x02\xF6W_\x80\xFD[\x80b\xFD\xD5\x8E\x14a\x01\xF8W\x80c\x01\xFF\xC9\xA7\x14a\x02AW\x80c\t[\xCD\xB6\x14a\x02pW\x80c\x0B\r\x9C\t\x14a\x02\x8FW[_\x80\xFD[4\x80\x15a\x02\x03W_\x80\xFD[Pa\x02.a\x02\x126`\x04aZ<V[`\x04` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02LW_\x80\xFD[Pa\x02`a\x02[6`\x04aZfV[a\x06\xD3V[`@Q\x90\x15\x15\x81R` \x01a\x028V[4\x80\x15a\x02{W_\x80\xFD[Pa\x02`a\x02\x8A6`\x04aZ\xA5V[a\x07kV[4\x80\x15a\x02\x9AW_\x80\xFD[Pa\x02\xAEa\x02\xA96`\x04aZ\xD7V[a\x08?V[\0[a\x02.a\x08\xC9V[4\x80\x15a\x02\xC3W_\x80\xFD[Pa\x02\xAEa\x02\xD26`\x04aZ\xA5V[a\t'V[4\x80\x15a\x02\xE2W_\x80\xFD[Pa\x02.a\x02\xF16`\x04a[\x15V[a\t\xABV[4\x80\x15a\x03\x01W_\x80\xFD[Pa\x02.a\x03\x106`\x04a\\\xCFV[a\t\xB5V[4\x80\x15a\x03 W_\x80\xFD[Pa\x02\xAEa\x03/6`\x04a]4V[a\n\xD9V[4\x80\x15a\x03?W_\x80\xFD[Pa\x03Sa\x03N6`\x04a]OV[a\x0B\xCCV[`@Qa\x028\x91\x90a]oV[a\x02.a\x03n6`\x04a]4V[a\x0C\tV[4\x80\x15a\x03~W_\x80\xFD[Pa\x02`a\x03\x8D6`\x04aZ\xA5V[a\x0CgV[4\x80\x15a\x03\x9DW_\x80\xFD[Pa\x03\xB1a\x03\xAC6`\x04a]\xB1V[a\x0C\xD8V[`@Qa\x028\x91\x90a]\xF0V[4\x80\x15a\x03\xC9W_\x80\xFD[Pa\x02\xAEa\x03\xD86`\x04a^CV[a\x0E*V[4\x80\x15a\x03\xE8W_\x80\xFD[Pa\x02`a\x03\xF76`\x04a^\x84V[a\x0E\xCCV[4\x80\x15a\x04\x07W_\x80\xFD[Pa\x02.a\x04\x166`\x04aZ\xD7V[`\x05` \x90\x81R_\x93\x84R`@\x80\x85 \x82R\x92\x84R\x82\x84 \x90R\x82R\x90 T\x81V[4\x80\x15a\x04CW_\x80\xFD[Pa\x04Wa\x04R6`\x04a^\xAEV[a\x0FfV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x028V[4\x80\x15a\x04wW_\x80\xFD[Pa\x04\x8Ba\x04\x866`\x04a_oV[a\x11eV[`@Q`\x02\x91\x90\x91\x0B\x81R` \x01a\x028V[4\x80\x15a\x04\xA9W_\x80\xFD[Pa\x02\xAEa\x04\xB86`\x04a^CV[a\x14\x12V[4\x80\x15a\x04\xC8W_\x80\xFD[Pa\x02\xAEa\x04\xD76`\x04aZ<V[a\x15\x03V[4\x80\x15a\x04\xE7W_\x80\xFD[Pa\x02.a\x04\xF66`\x04aZ\xD7V[a\x15\xC2V[4\x80\x15a\x05\x06W_\x80\xFD[P_Ta\x05&\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x028V[4\x80\x15a\x05VW_\x80\xFD[Pa\x02.a\x05e6`\x04a]4V[`\x01` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x05\x81W_\x80\xFD[Pa\x03Sa\x05\x906`\x04a_\xB7V[a\x16\xEEV[4\x80\x15a\x05\xA0W_\x80\xFD[Pa\x02\xAEa\x05\xAF6`\x04a]4V[a\x17'V[4\x80\x15a\x05\xBFW_\x80\xFD[Pa\x02`a\x05\xCE6`\x04a`(V[`\x03` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[4\x80\x15a\x05\xF8W_\x80\xFD[Pa\x03Sa\x06\x076`\x04a_\xB7V[a\x17\xCCV[4\x80\x15a\x06\x17W_\x80\xFD[P`\x02Ta\x05&\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x06CW_\x80\xFD[Pa\x02.a\x06R6`\x04a[\x15V[a\x18\x03V[4\x80\x15a\x06bW_\x80\xFD[Pa\x02\xAEa\x06q6`\x04a]4V[a\x18\rV[4\x80\x15a\x06\x81W_\x80\xFD[Pa\x02.a\x06\x906`\x04a`_V[a\x18\xFCV[4\x80\x15a\x06\xA0W_\x80\xFD[Pa\x02\xAEa\x06\xAF6`\x04aZ\xA5V[a\x1A\xAEV[4\x80\x15a\x06\xBFW_\x80\xFD[Pa\x02`a\x06\xCE6`\x04aa\x1DV[a\x1B2V[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14\x80a\x07eWP\x7F\x0Fc/\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[3_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x91\x90\x83\x90a\x07\x96\x90\x84\x90aa\x8DV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x07\xDA\x90\x84\x90aa\xA0V[\x90\x91UPP`@\x80Q3\x80\x82R` \x82\x01\x85\x90R\x85\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x92\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01[`@Q\x80\x91\x03\x90\xA4P`\x01[\x93\x92PPPV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x08\x8EWa\x08\x8E\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x08\xA3\x83a\x08\x9B\x83a\x1D&V[_\x033a\x1DkV[a\x08\xC4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\x1D\xCBV[PPPV[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\t\x19Wa\t\x19\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\t\"3a\x1E\xC6V[\x90P\x90V[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\tvWa\tv\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x81a\t\x84\x81a\x08\x9B\x84a\x1D&V[a\t\xA5\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x84a\x1F\xAFV[PPPPV[_\x81T_R` _\xF3[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\n\x05Wa\n\x05\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\n\ra NV[`\xA0\x86 _\x81\x81R`\x06` R`@\x90 a\n'\x81a \xB6V[`\x80\x88\x01Qa\nP\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x89\x89\x89a \xFDV[a\n[\x81\x88\x88a!\xE0V[\x92Pa\nh\x88\x843a\"\xDAV[`@\x80Q\x88\x81R` \x81\x01\x88\x90R3\x91\x84\x91\x7F)\xEF\x05\xCA\xAF\xF9@K|\xB6\xD1\xC0\xE9\xBB\xAE\x9E\xAAz\xB2T\x1F\xEB\xA1\xA9\xC4$\x85\x94\xC0\x81V\xCB\x91\x01`@Q\x80\x91\x03\x90\xA3`\x80\x88\x01Qa\n\xCE\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x89\x89\x89a#\x02V[PP\x95\x94PPPPPV[_Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0B^W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xB4\xBD\x8E\xF5=\xF6\x90\xB9\x94=3\x18\x99`\x06\xDB\xB8*%\xF5G\x19\xD8\xC8\x03[Qj*[\x8A\xCC\x90_\x90\xA2PV[```@Q\x80\x83`\x05\x1B` \x83R\x84` \x84\x01R`@\x83\x01\x92P\x80\x83\x01\x90P[\x85T\x83R` \x83\x01\x92P`\x01\x86\x01\x95P\x80\x83\x10a\x0B\xECW\x81\x81\x03\x82\xF3[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x0CYWa\x0CY\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x07e\x82a\x1E\xC6V[\x91\x90PV[3_\x81\x81R`\x05` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x80\x85R\x90\x83R\x81\x84 \x87\x85R\x90\x92R\x80\x83 \x85\x90UQ\x91\x92\x85\x92\x7F\xB3\xFDPq\x83X\x87Vz\x06q\x15\x11!\x89M\xDC\xCC(B\xF1\xD1\x0B\xED\xAD\x13\xE0\xD1|\xAC\xE9\xA7\x90a\x08,\x90\x87\x81R` \x01\x90V[``\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\\x15a\r*Wa\r*\x7FP\x90\xD6\xC6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\r2a#\xDAV[`@Q\x7F\x91\xDDsF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90c\x91\xDDsF\x90a\rp\x90\x86\x90\x86\x90`\x04\x01aa\xFAV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\r\x8BW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\r\xD0\x91\x90\x81\x01\x90ab\rV[\x90P\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B\\\x15a\x0E\"Wa\x0E\"\x7FR\x12\xCB\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x07ea$\0V[`@\x82\x01Qb\xFF\xFF\xFF\x16b\x80\0\0\x14\x15\x80a\x0EuWP\x81`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x0E\xA3Wa\x0E\xA3\x7F0\xD2\x16A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x0E\xB1\x81b\xFF\xFF\xFF\x16a$%V[`\xA0\x82 _\x81\x81R`\x06` R`@\x90 a\x08\xC4\x90\x83a$dV[3_\x81\x81R`\x03` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x80\x85R\x90\x83R\x81\x84 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x87\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x92\x93\x91\x7F\xCE\xB5v\xD9\xF1^N \x0F\xDBP\x96\xD6M]\xFDf~\x16\xDE\xF2\x0C\x1E\xEF\xD1BV\xD8\xE3\xFA\xA2g\x91\x01`@Q\x80\x91\x03\x90\xA3P`\x01\x92\x91PPV[_\x80\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x0F\xB7Wa\x0F\xB7\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x0F\xBFa NV[`\xA0\x86 _\x81\x81R`\x06` R`@\x90 a\x0F\xD9\x81a \xB6V[`\x80\x88\x01Qa\x10\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x89\x89a$\xBDV[_a\x10u`@Q\x80`\xC0\x01`@R\x803s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A_\x01Q`\x02\x0B\x81R` \x01\x8A` \x01Q`\x02\x0B\x81R` \x01a\x10R\x8B`@\x01Qa&~V[`\x0F\x0B\x81R``\x80\x8D\x01Q`\x02\x0B` \x83\x01R\x8B\x01Q`@\x90\x91\x01R\x83\x90a&\xB3V[\x94P\x90Pa\x10\x83\x81\x85a*\xF8V[\x94PPP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x7F\xF2\x08\xF4\x91'\x82\xFD%\xC7\xF1\x14\xCA7#\xA2\xD5\xDDo;\xCC:\xC8\xDBZ\xF6;\xAA\x85\xF7\x11\xD5\xEC\x88_\x01Q\x89` \x01Q\x8A`@\x01Q\x8B``\x01Q`@Qa\x11\x01\x94\x93\x92\x91\x90`\x02\x94\x85\x0B\x81R\x92\x90\x93\x0B` \x83\x01R`@\x82\x01R``\x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA3`\x80\x87\x01Q_\x90a\x115\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x87\x87\x8B\x8Ba++V[\x90\x94P\x90P\x80\x15a\x11OWa\x11O\x88\x82\x8A`\x80\x01Qa\"\xDAV[a\x11Z\x88\x853a\"\xDAV[PP\x94P\x94\x92PPPV[_a\x11na NV[``\x85\x01Qa\x7F\xFF`\x02\x91\x90\x91\x0B\x13\x15a\x11\xB2W``\x85\x01Qa\x11\xB2\x90\x7F\xB7\0$\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a-\x02V[`\x01`\x02\x0B\x85``\x01Q`\x02\x0B\x12\x15a\x11\xF5W``\x85\x01Qa\x11\xF5\x90\x7F\xE9\xE9\x05\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a-\x02V[\x84Q` \x86\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x10a\x12MW\x84Q` \x86\x01Qa\x12M\x91\x7Fnl\x980\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a-\x11V[a\x12~\x85`@\x01Q\x86`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a-T\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x12\xB2W`\x80\x85\x01Qa\x12\xB2\x90\x7F\xE6Z\xF6\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a.\"V[_a\x12\xC5\x86`@\x01Qb\xFF\xFF\xFF\x16a.DV[`\x80\x87\x01Q\x90\x91Pa\x12\xF0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x87\x87\x87a.iV[`\xA0\x86 _a\x12\xFE\x88a/@V[_\x83\x81R`\x06` R`@\x90 \x90\x91Pa\x13\x1A\x90\x88\x83\x86a0\xA8V[`\x80\x89\x01Q\x90\x94Pa\x13F\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x87\x8A\x8Aa1\x83V[\x87` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xDDFngN\xA5W\xF5b\x95\xE2\xD0!\x8A\x12^\xA4\xB4\xF0\xF6\xF30{\x95\xF8^a\x10\x83\x8Dd8\x8B`@\x01Q\x8C``\x01Q\x8D`\x80\x01Q\x8D\x8B`@Qa\x13\xFF\x95\x94\x93\x92\x91\x90b\xFF\xFF\xFF\x95\x90\x95\x16\x85R`\x02\x93\x84\x0B` \x86\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`@\x86\x01R\x91\x16``\x84\x01R\x90\x0B`\x80\x82\x01R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPP\x94\x93PPPPV[`\x02Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x14ZWa\x14Z\x7FH\xF5\xC3\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x03\xE9a\x0F\xFF\x82\x16\x10b>\x90\0b\xFF\xF0\0\x83\x16\x10\x16a\x14\xA2Wa\x14\xA2\x7F\xA7\xAB\xE2\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x83\x16a.\"V[`\xA0\x82 a\x14\xC4\x82a\x14\xBE\x83_\x90\x81R`\x06` R`@\x90 \x90V[\x90a2\\V[`@Qb\xFF\xFF\xFF\x83\x16\x81R\x81\x90\x7F\xE9\xC4%\x93\xE7\x1F\x84@;\x845,\xD1h\xD6\x93\xE2\xC9\xFC\xD1\xFD\xBC\xC3\xFE\xB2\x1D\x92\xB4>f\x96\xF9\x90` \x01`@Q\x80\x91\x03\x90\xA2PPPV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x15RWa\x15R\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[3_\x90\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` R`@\x81 \\\x90a\x15\x80\x83a\x1D&V[\x90P\x81\x81`\x0F\x0B\x14a\x15\xB5Wa\x15\xB5\x7F\xBD\xA7:\xBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\t\xA5\x84\x82_\x033a\x1DkV[`\x02T_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x16\x0CWa\x16\x0C\x7FH\xF5\xC3\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\\x15a\x16\\Wa\x16\\\x7F>_O\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x81\x15a\x16hW\x81a\x16\x8EV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x01` R`@\x90 T[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x90\x81R`\x01` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a\x16\xC7\x90\x84\x90aa\x8DV[\x90\x91UPa\x088\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x85\x83a\x1D\xCBV[```@Q\x80` \x82R\x83` \x83\x01R`@\x82\x01\x91P\x83`\x05\x1B\x82\x01\x85[\x805\\\x84R` \x93\x84\x01\x93\x01\x81\x84\x10a\x17\x0CW[P\x81\x81\x03\x82\xF3[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x17vWa\x17v\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x17\x9CWa\x17\x99a2\xB0V[PV[_a\x17\xBC\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a2\xD5V[\x90Pa\x17\xC8\x82\x82a3\x84V[PPV[```@Q\x80` \x82R\x83` \x83\x01R`@\x82\x01\x91P\x83`\x05\x1B\x82\x01\x85[\x805T\x84R` \x93\x84\x01\x93\x01\x81\x84\x10\x15a\x17 Wa\x17\xEAV[_\x81\\_R` _\xF3[_Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x18\x8DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0BUV[_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x19LWa\x19L\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x19Ta NV[\x83` \x01Q_\x03a\x19\x88Wa\x19\x88\x7F\xBE\x8B\x85\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[`\xA0\x85 _\x81\x81R`\x06` R`@\x90 a\x19\xA2\x81a \xB6V[`\x80\x87\x01Q_\x90\x81\x90\x81\x90a\x19\xD0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8B\x8B\x8B\x8Ba3\xE4V[\x80\x93P\x81\x95P\x82\x94PPPPa\x1AL\x84\x86`@Q\x80`\xA0\x01`@R\x80\x86\x81R` \x01\x8E``\x01Q`\x02\x0B\x81R` \x01\x8D_\x01Q\x15\x15\x81R` \x01\x8D`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85b\xFF\xFF\xFF\x16\x81RP\x8C_\x01Qa\x1AEW\x8D` \x01Qa5\x8BV[\x8DQa5\x8BV[`\x80\x8B\x01Q\x90\x96P_\x92Pa\x1A}\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8A\x8A\x88\x8B\x8B\x88a6\x8CV[\x90\x95P\x90P\x80\x15a\x1A\x97Wa\x1A\x97\x89\x82\x8B`\x80\x01Qa\"\xDAV[a\x1A\xA2\x89\x863a\"\xDAV[PPPP\x94\x93PPPPV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x1A\xFDWa\x1A\xFD\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x81a\x1B\x11\x81a\x1B\x0B\x84a\x1D&V[3a\x1DkV[a\t\xA5\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x84a8%V[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x14\x80\x15\x90a\x1B\x89WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16\x15[\x15a\x1C2Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x86\x84R\x90\x91R\x90 T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x1C0Wa\x1B\xF7\x83\x82aa\x8DV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x88\x84R\x90\x91R\x90 U[P[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x1Cq\x90\x84\x90aa\x8DV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x1C\xB5\x90\x84\x90aa\xA0V[\x90\x91UPP`@\x80Q3\x81R` \x81\x01\x84\x90R\x84\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x92\x90\x89\x16\x91\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01`@Q\x80\x91\x03\x90\xA4P`\x01[\x94\x93PPPPV[\x80_R`\x04_\xFD[_o\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x1DgWa\x1Dg\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[P\x90V[\x81`\x0F\x0B_\x03a\x1DzWPPPV[_\x80a\x1D\x9Ds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84\x86a96V[\x91P\x91P\x80_\x03a\x1D\xB5Wa\x1D\xB0a9|V[a\x1D\xC4V[\x81_\x03a\x1D\xC4Wa\x1D\xC4a9\xCAV[PPPPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x1E%W_\x80_\x80\x85\x87Z\xF1\x90P\x80a\x1E Wa\x1E \x7F\x85I\xDBY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a:\x18V[a\t\xA5V[`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` _`D\x83_\x89Z\xF1=\x15`\x1F=\x11`\x01_Q\x14\x16\x17\x16\x91P_\x81R_` \x82\x01R_`@\x82\x01RP\x80a\t\xA5Wa\t\xA5\x7F\xB1,_\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a:\x18V[_\x7F'\xE0\x98\xC5\x05\xD4N\xC3W@\x04\xBC\xA0R\xAA\xBFv\xBD5\0L\x18 \x99\xD8\xC5u\xFB#\x85\x93\xB9\\s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x1F\x0CW4\x91Pa\x1F\x96V[4\x15a\x1F;Wa\x1F;\x7F\xB0\xEC\x84\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x7F\x1E\x07E\xA7\xDB\x16#\x98\x1F\x0B*]B26L\0xrf\xEBu\xADTo\x19\x0El\xEB\xE9\xBD\x95\\_a\x1F}s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a2\xD5V[\x90Pa\x1F\x89\x82\x82aa\x8DV[\x93Pa\x1F\x93a2\xB0V[PP[a\x1F\xA9\x81a\x1F\xA3\x84a\x1D&V[\x85a\x1DkV[P\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x1F\xEE\x90\x84\x90aa\xA0V[\x90\x91UPP`@\x80Q3\x81R` \x81\x01\x83\x90R\x83\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x91_\x91\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01[`@Q\x80\x91\x03\x90\xA4PPPV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a \xB4Wa \xB4\x7F\r\x89C\x8E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[V[\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\x17\x99Wa\x17\x99\x7FHj\xA3\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x853s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a!\xD7W` \x87\x16\x15a!\xD7Wa!\xD53\x87\x87\x87\x87\x87`@Q`$\x01a!B\x96\x95\x94\x93\x92\x91\x90ab\xBFV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\xA8\xB0\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a:KV[P[PPPPPPPV[`\x03\x83\x01T_\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x82\x03a\")Wa\")\x7F\xA7O\x97\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\"`a\"5\x85a\x1D&V[_\x03a\"@\x85a\x1D&V[_\x03`\x80\x91\x90\x91\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x90V[\x91P\x83\x15a\"\x9AW`\x01\x85\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x02\x04\x01\x90U[\x82\x15a\"\xD2W`\x02\x85\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x02\x04\x01\x90U[P\x93\x92PPPV[\x82Qa\"\xF0\x90a\"\xEA\x84`\x80\x1D\x90V[\x83a\x1DkV[a\x08\xC4\x83` \x01Qa\"\xEA\x84`\x0F\x0B\x90V[\x853s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a!\xD7W`\x10\x87\x16\x15a!\xD7Wa!\xD53\x87\x87\x87\x87\x87`@Q`$\x01a#G\x96\x95\x94\x93\x92\x91\x90ab\xBFV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE1\xB4\xAFi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a:KV[`\x01\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#]V[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#]V[b\x0FB@b\xFF\xFF\xFF\x82\x16\x11\x15a\x17\x99Wa\x17\x99\x7F\x14\0!\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x83\x16a.\"V[a$m\x82a \xB6V[\x81T\x7F\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16|\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xD0\x83\x90\x1B\x16\x17[\x90\x91UPV[\x843s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a&vW_\x84`@\x01Q\x13\x80\x15a$\xF1WPa\x08\0\x86\x16\x15\x15[\x15a%\xABWa%\xA53\x86\x86\x86\x86`@Q`$\x01a%\x12\x95\x94\x93\x92\x91\x90ac\x8BV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90a:KV[Pa&vV[_\x84`@\x01Q\x13\x15\x80\x15a%\xC2WPa\x02\0\x86\x16\x15\x15[\x15a&vWa!\xD73\x86\x86\x86\x86`@Q`$\x01a%\xE3\x95\x94\x93\x92\x91\x90ac\x8BV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90a:KV[PPPPPPV[\x80`\x0F\x81\x90\x0B\x81\x14a\x0CbWa\x0Cb\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[``\x81\x01Q` \x82\x01Q`@\x83\x01Q_\x92\x83\x92\x90\x91a&\xD2\x82\x82a;>V[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R\x83`\x0F\x0B_\x14a(\xA2Wa'\x0B\x88\x84\x86_a<\x05V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x83\x01R\x15\x15\x81Ra'3\x88\x83\x86`\x01a<\x05V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x83\x01R\x15\x15`@\x82\x01R_`\x0F\x85\x90\x0B\x12a(gW`\x80\x87\x01Q_\x90`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18\x81\x81\x07\x83\x13\x90\x82\x90\x05\x03b\r\x89\xE8\x91\x90\x91\x05\x03`\x01\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82` \x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a(\x0BWa(\x0B\x7F\xB8\xE3\xC3\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a-\x02V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82``\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a(eWa(e\x7F\xB8\xE3\xC3\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a-\x02V[P[\x80Q\x15a(\x83W`\x80\x87\x01Qa(\x83\x90`\x05\x8A\x01\x90\x85\x90a<\xEEV[\x80`@\x01Q\x15a(\xA2W`\x80\x87\x01Qa(\xA2\x90`\x05\x8A\x01\x90\x84\x90a<\xEEV[_\x80a(\xAF\x8A\x86\x86a=@V[\x8AQ`\xA0\x8C\x01Q`@\x80Q`&\x81\x01\x92\x90\x92R`\x06\x80\x83\x01\x8A\x90R`\x03\x83\x01\x8B\x90R\x92\x82R`:`\x0C\x83\x01 _\x83\x83\x01\x81\x90R` \x80\x85\x01\x82\x90R\x93\x81\x90R\x90\x81R\x92\x8F\x01\x90\x91R\x81 \x92\x94P\x90\x92P\x80a)\x0C\x83\x8A\x87\x87a=\xF4V[\x91P\x91Pa)Aa)\x1C\x83a\x1D&V[a)%\x83a\x1D&V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x17\x90V[\x99PPPPPP_\x84`\x0F\x0B\x12\x15a)\xAAW\x80Q\x15a)}W`\x02\x83\x81\x0B_\x90\x81R`\x04\x8A\x01` R`@\x81 \x81\x81U`\x01\x81\x01\x82\x90U\x90\x91\x01U[\x80`@\x01Q\x15a)\xAAW`\x02\x82\x81\x0B_\x90\x81R`\x04\x8A\x01` R`@\x81 \x81\x81U`\x01\x81\x01\x82\x90U\x90\x91\x01U[P\x82`\x0F\x0B_\x14a*\xEEW\x86T_\x80a)\xC6\x83`\xA0\x1C`\x02\x0B\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x91P\x91P\x84`\x02\x0B\x82`\x02\x0B\x12\x15a*\"Wa*\x1Ba*\x15a*\x10a*\x01\x88a?%V[a*\n\x88a?%V[\x8AaB\"V[a&~V[`\x80\x1B\x90V[\x97Pa*\xEAV[\x83`\x02\x0B\x82`\x02\x0B\x12\x15a*\xC5Wa*Ya*Ca*\x10\x83a*\n\x88a?%V[a)%a*\x10a*R\x89a?%V[\x85\x8BaBZV[`\x03\x8B\x01T\x90\x98Pa*}\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87aB\x86V[`\x03\x8B\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua*\xEAV[a*\xE7_a)%a*\x10a*\xD8\x89a?%V[a*\xE1\x89a?%V[\x8BaBZV[\x97P[PPP[PPP\x92P\x92\x90PV[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x01`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x01a+\"a+\x19\x83a&~V[a)%\x83a&~V[\x95\x94PPPPPV[_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x163\x03a+TWP\x84\x90P_a,\xF6V[\x85\x91P_\x87`@\x01Q\x13\x15a,<Wa\x04\0\x89\x16\x15a,7Wa,(3\x89\x89\x89\x89\x89\x89`@Q`$\x01a+\x8D\x97\x96\x95\x94\x93\x92\x91\x90admV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x9F\x06>\xFC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x02\x8B\x16\x15\x15[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x91\x90aB\xB6V[\x90Pa,4\x82\x82aC\x10V[\x91P[a,\xF6V[a\x01\0\x89\x16\x15a,\xF6Wa,\xE73\x89\x89\x89\x89\x89\x89`@Q`$\x01a,f\x97\x96\x95\x94\x93\x92\x91\x90admV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fl+\xBE~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x01\x8B\x16\x15\x15a,\nV[\x90Pa,\xF3\x82\x82aC\x10V[\x91P[\x97P\x97\x95PPPPPPV[\x81_R\x80`\x02\x0B`\x04R`$_\xFD[`@Q\x83\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x81\xFD[_`\x80\x83\x16\x15\x80\x15a-hWP`\x08\x83\x16\x15\x15[\x15a-tWP_a\x07eV[`@\x83\x16\x15\x80\x15a-\x87WP`\x04\x83\x16\x15\x15[\x15a-\x93WP_a\x07eV[a\x04\0\x83\x16\x15\x80\x15a-\xA7WP`\x02\x83\x16\x15\x15[\x15a-\xB3WP_a\x07eV[a\x01\0\x83\x16\x15\x80\x15a-\xC7WP`\x01\x83\x16\x15\x15[\x15a-\xD3WP_a\x07eV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x15a.\x11Wa?\xFF\x83\x16\x15\x15\x80a.\x0CWPb\x80\0\0b\xFF\xFF\xFF\x83\x16\x14a\x088V[a\x088V[Pb\xFF\xFF\xFF\x16b\x80\0\0\x14\x15\x91\x90PV[\x81_Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\x04R`$_\xFD[_b\x80\0\0b\xFF\xFF\xFF\x83\x16\x03a.[WP_\x91\x90PV[a\x1Dg\x82b\xFF\xFF\xFF\x16a$%V[\x843s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a&vWa \0\x86\x16\x15a&vWa!\xD73\x86\x86\x86\x86`@Q`$\x01a.\xAD\x95\x94\x93\x92\x91\x90ae]V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F4@\xD8 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90a:KV[`\x02T_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x0CbW_a/lE`daC1V[\x90P\x80Z\x10\x15a/\x9FWa/\x9F\x7F\x1E\xE4\x97\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[`\x02T`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90_\x90a/\xCE\x90\x86\x90`$\x01af,V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x80\x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FU;\xFC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x81R\x82Q\x92\x93P_\x92\x83\x92\x91\x83\x91\x90\x82\x88\x8A\xF1_Q=` \x14\x90\x91\x16\x92P\x90P\x81\x80\x15a0tWP\x80b\xFF\xFF\xFF\x16\x81\x14[\x80\x15a0\x91WPa\x03\xE9a\x0F\xFF\x82\x16\x10b>\x90\0b\xFF\xF0\0\x83\x16\x10\x16[a0\x9BW_a0\x9DV[\x80[\x97\x96PPPPPPPV[\x83T_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a0\xF0Wa0\xF0\x7Fy\x83\xC0Q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a0\xF9\x84aC\x85V[\x90P|\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xD0\x83\x90\x1B\x16v\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xA0\x83\x90\x1B\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x17`\xB8\x85\x90\x1By\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x17\x90\x94UP\x91\x92\x91PPV[\x853s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a!\xD7Wa\x10\0\x87\x16\x15a!\xD7Wa!\xD53\x87\x87\x87\x87\x87`@Q`$\x01a1\xC9\x96\x95\x94\x93\x92\x91\x90af\xAFV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xA9\x10\xF8\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a:KV[a2e\x82a \xB6V[\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16y\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xB8\x83\x90\x1B\x16\x17a$\xB7V[_\x7F'\xE0\x98\xC5\x05\xD4N\xC3W@\x04\xBC\xA0R\xAA\xBFv\xBD5\0L\x18 \x99\xD8\xC5u\xFB#\x85\x93\xB9]V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a2\xF8WPG\x91\x90PV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3`W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07e\x91\x90ag\x88V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x7F'\xE0\x98\xC5\x05\xD4N\xC3W@\x04\xBC\xA0R\xAA\xBFv\xBD5\0L\x18 \x99\xD8\xC5u\xFB#\x85\x93\xB9]\x80\x7F\x1E\x07E\xA7\xDB\x16#\x98\x1F\x0B*]B26L\0xrf\xEBu\xADTo\x19\x0El\xEB\xE9\xBD\x95]PPV[` \x83\x01Q_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x163\x03a4\x10W_\x91Pa5\x80V[`\x80\x88\x16\x15a5\x80W_a4\xB2\x893\x8A\x8A\x8A\x8A`@Q`$\x01a47\x95\x94\x93\x92\x91\x90ag\x9FV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FW^$\xB4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra:KV[\x90P\x80Q``\x14a4\xE6Wa4\xE6\x7F\x1E\x04\x8E\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[`@\x88\x01Qb\xFF\xFF\xFF\x16b\x80\0\0\x03a5\x01W``\x81\x01Q\x91P[`\x08\x89\x16\x15a5~W`@\x81\x01Q\x92P_a5\x1C\x84`\x80\x1D\x90V[\x90P\x80`\x0F\x0B_\x14a5|W_\x85\x12a59`\x0F\x83\x90\x0B\x87ah\x87V[\x95P\x80a5HW_\x86\x12a5LV[_\x86\x13[\x15a5zWa5z\x7F\xFA\x0Bq\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[P[P[P[\x95P\x95P\x95\x92PPPV[_\x80\x80\x80\x80a5\x9A\x89\x88aF\xA9V[\x93P\x93P\x93P\x93P_\x83\x11\x15a5\xD5Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16_\x90\x81R`\x01` R`@\x90 \x80T\x84\x01\x90U[3\x88\x7F@\xE9\xCE\xCB\x9F_\x1F\x1C[\x9C\x97\xDE\xC2\x91{~\xE9.W\xBAUcp\x8D\xAC\xA9M\xD8J\xD7\x11/a6\x02\x87`\x80\x1D\x90V[a6\x0C\x88`\x0F\x0B\x90V[\x85Q`@\x80\x88\x01Q` \x80\x8A\x01Q\x83Q`\x0F\x97\x88\x0B\x81R\x95\x90\x96\x0B\x90\x85\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x90\x83\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x82\x01R`\x02\x91\x90\x91\x0B`\x80\x82\x01Rb\xFF\xFF\xFF\x86\x16`\xA0\x82\x01R`\xC0\x01`@Q\x80\x91\x03\x90\xA3P\x91\x97\x96PPPPPPPV[_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x163\x03a6\xB5WP\x84\x90P_a,\xF6V[_a6\xC0\x84`\x80\x1D\x90V[\x90P_a6\xCD\x85`\x0F\x0B\x90V[\x90P`@\x8B\x16\x15a7\xA0Wa7\x93a*\x103\x8C\x8C\x8C\x8C\x8C`@Q`$\x01a6\xF9\x96\x95\x94\x93\x92\x91\x90ah\xAEV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB4{/\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x04\x8E\x16\x15\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8F\x16\x91\x90aB\xB6V[a7\x9D\x90\x82ai\x9DV[\x90P[_\x81`\x0F\x0B_\x14\x15\x80a7\xB6WP\x82`\x0F\x0B_\x14\x15[\x15a8\x13W\x89Q` \x8B\x01Q_\x13\x90\x15\x15\x14a7\xEAWo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x80\x83\x90\x1B\x17a8\x04V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x80\x84\x90\x1B\x17[\x90Pa8\x10\x89\x82aC\x10V[\x98P[\x97\x9B\x97\x9AP\x96\x98PPPPPPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81\x14\x80\x15\x90a8~WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R T`\xFF\x16\x15[\x15a9+Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x86\x83R\x90R T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a9)Wa8\xEE\x83\x82aa\x8DV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x81R\x82\x82 \x88\x83R\x90R U[P[a\t\xA5\x84\x84\x84aO;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16_\x90\x81R\x90\x84\x16` R`@\x81 \x80\\\x91\x90a9n`\x0F\x85\x90\x0B\x84ah\x87V[\x91P\x81\x81]P\x93P\x93\x91PPV[\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B\\`\x01\x81\x03\x90P\x80\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B]PV[\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B\\`\x01\x81\x01\x90P\x80\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B]PV[=`@Q\x83\x81R\x82`\x04\x82\x01R`@`$\x82\x01R\x81`D\x82\x01R\x81_`d\x83\x01>` \x80`\x1F\x84\x01\x04\x02`d\x01\x91P\x81\x81\xFD[``_\x80_\x84Q` \x86\x01_\x88Z\xF1\x90P\x80a:\x8BWa:\x8B\x7F1\x9DT\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a:\x18V[`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>` \x82Q\x10\x80a;\tWP` \x83\x01Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a:\xE4\x83` \x01Q\x90V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15[\x15a;7Wa;7\x7F\x1E\x04\x8E\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[P\x92\x91PPV[\x80`\x02\x0B\x82`\x02\x0B\x12a;vWa;v\x7F\xC4C>\xD5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x83aO\xD1V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18`\x02\x83\x90\x0B\x12\x15a;\xCCWa;\xCC\x7F\xD5\xE2\xF7\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a-\x02V[b\r\x89\xE8`\x02\x82\x90\x0B\x13\x15a\x17\xC8Wa\x17\xC8\x7F\x1A\xD7w\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a-\x02V[`\x02\x83\x90\x0B_\x90\x81R`\x04\x85\x01` R`@\x81 \x80T\x82\x91\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\x0F\x0Ba<S\x82\x88aB\x86V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x15\x90\x84\x16\x80\x15\x91\x90\x91\x14\x15\x96P\x90\x94P_\x03a<\xA6W\x88T`\xA0\x1C`\x02\x0B`\x02\x0B\x88`\x02\x0B\x13a<\xA6W`\x01\x80\x8A\x01T\x90\x84\x01U`\x02\x80\x8A\x01T\x90\x84\x01U[_\x86a<\xBBWa<\xB6\x88\x83ai\x9DV[a<\xC5V[a<\xC5\x88\x83ai\xEBV[\x90P\x80`\x80\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x17\x84UPPPP\x94P\x94\x92PPPV[`\x02\x91\x82\x0B\x91\x0B\x80\x82\x07\x15a=\x1BW`@Qc\xD4\xD8\xF3\xE6\x81R\x82` \x82\x01R\x81`@\x82\x01R`D`\x1C\x82\x01\xFD[\x80\x82\x05\x91P\x81`\x08\x1D_R\x82` R`@_ `\x01`\xFF\x84\x16\x1B\x81T\x18\x81UPPPPV[`\x02\x82\x81\x0B_\x81\x81R`\x04\x86\x01` R`@\x80\x82 \x85\x85\x0B\x83R\x90\x82 \x87T\x92\x94\x85\x94\x92\x93\x91\x92`\xA0\x92\x90\x92\x1C\x90\x0B\x90\x81\x12\x15a=\x96W\x81`\x01\x01T\x83`\x01\x01T\x03\x94P\x81`\x02\x01T\x83`\x02\x01T\x03\x93Pa=\xE9V[\x85`\x02\x0B\x81`\x02\x0B\x12a=\xC2W\x82`\x01\x01T\x82`\x01\x01T\x03\x94P\x82`\x02\x01T\x82`\x02\x01T\x03\x93Pa=\xE9V[\x81`\x01\x01T\x83`\x01\x01T\x89`\x01\x01T\x03\x03\x94P\x81`\x02\x01T\x83`\x02\x01T\x89`\x02\x01T\x03\x03\x93P[PPP\x93P\x93\x91PPV[\x83T_\x90\x81\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x0F\x86\x90\x0B\x82\x03a>^W\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a>YWa>Y\x7F\xAE\xFE\xB9$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a>\xA5V[a>h\x81\x87aB\x86V[\x87T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x87U[a>\xD9\x87`\x01\x01T\x86\x03\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aO\xEEV[\x92Pa?\x0F\x87`\x02\x01T\x85\x03\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aO\xEEV[`\x01\x88\x01\x95\x90\x95UPP`\x02\x90\x94\x01U\x91\x92\x90PV[`\x02\x0B_`\xFF\x82\x90\x1D\x80\x83\x01\x18b\r\x89\xE8\x81\x11\x15a?gWa?g\x7F\x8B\x862z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a-\x02V[p\x01\xFF\xFC\xB93\xBDo\xAD7\xAA-\x16-\x1AY@\x01`\x01\x82\x16\x02p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18`\x02\x82\x16\x15a?\xB0Wo\xFF\xF9rr7=A2Y\xA4i\x90X\x0E!:\x02`\x80\x1C[`\x04\x82\x16\x15a?\xCFWo\xFF\xF2\xE5\x0F_ei2\xEF\x125|\xF3\xC7\xFD\xCC\x02`\x80\x1C[`\x08\x82\x16\x15a?\xEEWo\xFF\xE5\xCA\xCA~\x10\xE4\xE6\x1C6$\xEA\xA0\x94\x1C\xD0\x02`\x80\x1C[`\x10\x82\x16\x15a@\rWo\xFF\xCB\x98C\xD6\x0FaY\xC9\xDBX\x83\\\x92fD\x02`\x80\x1C[` \x82\x16\x15a@,Wo\xFF\x97;A\xFA\x98\xC0\x81G.h\x96\xDF\xB2T\xC0\x02`\x80\x1C[`@\x82\x16\x15a@KWo\xFF.\xA1df\xC9j8C\xECx\xB3&\xB5(a\x02`\x80\x1C[`\x80\x82\x16\x15a@jWo\xFE]\xEE\x04j\x99\xA2\xA8\x11\xC4a\xF1\x96\x9C0S\x02`\x80\x1C[a\x01\0\x82\x16\x15a@\x8AWo\xFC\xBE\x86\xC7\x90\n\x88\xAE\xDC\xFF\xC8;G\x9A\xA3\xA4\x02`\x80\x1C[a\x02\0\x82\x16\x15a@\xAAWo\xF9\x87\xA7%:\xC4\x13\x17o+\x07L\xF7\x81^T\x02`\x80\x1C[a\x04\0\x82\x16\x15a@\xCAWo\xF39+\x08\"\xB7\0\x05\x94\x0Cz9\x8EKp\xF3\x02`\x80\x1C[a\x08\0\x82\x16\x15a@\xEAWo\xE7\x15\x94u\xA2\xC2\x9BtC\xB2\x9C\x7F\xA6\xE8\x89\xD9\x02`\x80\x1C[a\x10\0\x82\x16\x15aA\nWo\xD0\x97\xF3\xBD\xFD \"\xB8\x84Z\xD8\xF7\x92\xAAX%\x02`\x80\x1C[a \0\x82\x16\x15aA*Wo\xA9\xF7FF-\x87\x0F\xDF\x8Ae\xDC\x1F\x90\xE0a\xE5\x02`\x80\x1C[a@\0\x82\x16\x15aAJWop\xD8i\xA1V\xD2\xA1\xB8\x90\xBB=\xF6+\xAF2\xF7\x02`\x80\x1C[a\x80\0\x82\x16\x15aAjWo1\xBE\x13_\x97\xD0\x8F\xD9\x81#\x15\x05T/\xCF\xA6\x02`\x80\x1C[b\x01\0\0\x82\x16\x15aA\x8BWo\t\xAAP\x8B[z\x84\xE1\xC6w\xDET\xF3\xE9\x9B\xC9\x02`\x80\x1C[b\x02\0\0\x82\x16\x15aA\xABWn]j\xF8\xDE\xDB\x81\x19f\x99\xC3)\"^\xE6\x04\x02`\x80\x1C[b\x04\0\0\x82\x16\x15aA\xCAWm\"\x16\xE5\x84\xF5\xFA\x1E\xA9&\x04\x1B\xED\xFE\x98\x02`\x80\x1C[b\x08\0\0\x82\x16\x15aA\xE7Wk\x04\x8A\x17\x03\x91\xF7\xDCBDN\x8F\xA2\x02`\x80\x1C[_\x84\x13\x15aB\x12W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04[c\xFF\xFF\xFF\xFF\x01` \x1C\x93\x92PPPV[_\x80\x82`\x0F\x0B\x12aBIWaBBaB=\x85\x85\x85`\x01aP\xA9V[aQ\xDBV[_\x03a\x1D\x16V[a\x1D\x16aB=\x85\x85\x85_\x03_aP\xA9V[_\x80\x82`\x0F\x0B\x12aBuWaBBaB=\x85\x85\x85`\x01aR\rV[a\x1D\x16aB=\x85\x85\x85_\x03_aR\rV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x0F\x82\x90\x0B\x01`\x80\x81\x90\x1C\x15a\x07eWc\x93\xDA\xFD\xF1_R`\x04`\x1C\xFD[_\x80aB\xC2\x85\x85a:KV[\x90P\x82aB\xD2W_\x91PPa\x088V[\x80Q`@\x14aC\x04WaC\x04\x7F\x1E\x04\x8E\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[`@\x01Q\x94\x93PPPPV[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x03`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x03a+\"a+\x19\x83a&~V[_a'\x10\x82\x11\x15aCnW`@Q\x7F\xDE\xAA\x01\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a'\x10aC{\x83\x85aj9V[a\x088\x91\x90aj}V[_s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DQc\x96\x16\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xFF\xFD\x89]\x83\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aD\x04WaD\x04\x7FaHu$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a.\"V[w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0` \x83\x90\x1B\x16\x80_aD.\x82aRxV[`\xFF\x16\x90P`\x80\x81\x10aDIW`\x7F\x81\x03\x83\x90\x1C\x91PaDSV[\x80`\x7F\x03\x83\x90\x1B\x91P[\x90\x80\x02`\x7F\x81\x81\x1C`\xFF\x83\x81\x1C\x91\x90\x91\x1C\x80\x02\x80\x83\x1C\x81\x83\x1C\x1C\x80\x02\x80\x84\x1C\x81\x84\x1C\x1C\x80\x02\x80\x85\x1C\x81\x85\x1C\x1C\x80\x02\x80\x86\x1C\x81\x86\x1C\x1C\x80\x02\x80\x87\x1C\x81\x87\x1C\x1C\x80\x02\x80\x88\x1C\x81\x88\x1C\x1C\x80\x02\x80\x89\x1C\x81\x89\x1C\x1C\x80\x02\x80\x8A\x1C\x81\x8A\x1C\x1C\x80\x02\x80\x8B\x1C\x81\x8B\x1C\x1C\x80\x02\x80\x8C\x1C\x81\x8C\x1C\x1C\x80\x02\x80\x8D\x1C\x81\x8D\x1C\x1C\x80\x02\x80\x8E\x1C\x9C\x81\x90\x1C\x9C\x90\x9C\x1C\x80\x02\x9C\x8D\x90\x1C\x9E\x9D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8F\x01`@\x1B`\xC0\x91\x90\x91\x1Cg\x80\0\0\0\0\0\0\0\x16\x17`\xC1\x9B\x90\x9B\x1Cg@\0\0\0\0\0\0\0\x16\x9A\x90\x9A\x17`\xC2\x99\x90\x99\x1Cg \0\0\0\0\0\0\0\x16\x98\x90\x98\x17`\xC3\x97\x90\x97\x1Cg\x10\0\0\0\0\0\0\0\x16\x96\x90\x96\x17`\xC4\x95\x90\x95\x1Cg\x08\0\0\0\0\0\0\0\x16\x94\x90\x94\x17`\xC5\x93\x90\x93\x1Cg\x04\0\0\0\0\0\0\0\x16\x92\x90\x92\x17`\xC6\x91\x90\x91\x1Cg\x02\0\0\0\0\0\0\0\x16\x17`\xC7\x91\x90\x91\x1Cg\x01\0\0\0\0\0\0\0\x16\x17`\xC8\x91\x90\x91\x1Cf\x80\0\0\0\0\0\0\x16\x17`\xC9\x91\x90\x91\x1Cf@\0\0\0\0\0\0\x16\x17`\xCA\x91\x90\x91\x1Cf \0\0\0\0\0\0\x16\x17`\xCB\x91\x90\x91\x1Cf\x10\0\0\0\0\0\0\x16\x17`\xCC\x91\x90\x91\x1Cf\x08\0\0\0\0\0\0\x16\x17`\xCD\x91\x90\x91\x1Cf\x04\0\0\0\0\0\0\x16\x17i6'\xA3\x01\xD7\x10UwL\x85\x81\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFDp\x9B~T\x80\xFB\xA5\xA5\x0F\xED^b\xFF\xC5V\x81\x01`\x80\x90\x81\x1D\x90o\xDB-\xF0\x9E\x81\x95\x9A\x81E^&\x07\x99\xA0c/\x83\x01\x90\x1D`\x02\x81\x81\x0B\x90\x83\x90\x0B\x14aF\x9AW\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aFr\x82a?%V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aF\x94W\x81aF\x9CV[\x80aF\x9CV[\x81[\x99\x98PPPPPPPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x81\x90\x81\x90\x85T`@\x86\x01Q_\x81aF\xE3Wa\x0F\xFF`\xC4\x84\x90\x1C\x16aF\xEDV[a\x0F\xFF`\xB8\x84\x90\x1C\x16[\x88Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x86Ra\xFF\xFF\x91\x90\x91\x16\x91P_`\xA0\x85\x90\x1C`\x02\x0B`\x02\x0B` \x87\x01R`\x03\x8B\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x87\x01R`\x80\x8A\x01Q_\x90b@\0\0\x16aG]W`\xD0\x86\x90\x1Cb\xFF\xFF\xFF\x16aGoV[aGo\x8B`\x80\x01Qb\xFF\xFF\xFF\x16aS\x0CV[\x90P\x83\x15aG\x9DWb\x0FB@a\x0F\xFF\x85\x16b\xFF\xFF\xFF\x83\x16\x81\x81\x02\x83\x81\x06\x15\x15\x93\x90\x04\x92\x90\x92\x01\x91\x01\x03aG\x9FV[\x80[\x97PPb\x0FB@\x87b\xFF\xFF\xFF\x16\x10aG\xE3W\x89Q_\x12\x15aG\xE3WaG\xE3\x7F\x96 bF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x89Q_\x03aG\xFBW_\x80\x98P\x98PPPPPPaO2V[\x83\x15aH\xDEW``\x8A\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x16\x10aHmWaHms\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16[``\x8C\x01Q\x7F|\x9Cn\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90a-\x11V[d\x01\0\x02v\xA3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8A``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11aH\xD9W``\x8A\x01QaH\xD9\x90\x7F\x9EM|\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a.\"V[aI\x9CV[``\x8A\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x16\x11aI!WaI!s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16aH@V[s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8A``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10aI\x9CW``\x8A\x01QaI\x9C\x90\x7F\x9EM|\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a.\"V[`@\x80Qa\x01\0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x84aI\xEBW\x8B`\x02\x01TaI\xF1V[\x8B`\x01\x01T[`\xE0\x82\x01R[\x82\x15\x80aJ6WP\x8A``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[aM\xBEW\x86Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x80\x88\x01Q\x90\x8C\x01QaJm\x91`\x05\x8F\x01\x91\x88aS\x1BV[\x15\x15`@\x83\x01R`\x02\x0B` \x82\x01\x81\x90R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18\x12aJ\xCBW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18` \x82\x01R[b\r\x89\xE8`\x02\x0B\x81` \x01Q`\x02\x0B\x12aJ\xE9Wb\r\x89\xE8` \x82\x01R[aJ\xF6\x81` \x01Qa?%V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x81\x01\x82\x90R\x89Q\x90\x8E\x01QaK@\x93\x91\x92\x91\x16\x80\x82\x18\x91\x81\x11`\x01\x8A\x16\x18\x91\x90\x91\x02\x18\x89`@\x01Q\x86\x8CaTFV[`\xC0\x85\x01R`\xA0\x84\x01R`\x80\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87R\x8AQ_\x12\x15aK\xA9WaK}\x81`\xA0\x01QaQ\xDBV[\x83\x03\x92PaK\x98\x81`\xC0\x01Q\x82`\x80\x01QaB=\x91\x90aa\xA0V[aK\xA2\x90\x83aj\xB5V[\x91PaK\xDAV[aK\xBC\x81`\xC0\x01Q\x82`\x80\x01Q\x01aQ\xDBV[\x83\x01\x92PaK\xCD\x81`\xA0\x01QaQ\xDBV[aK\xD7\x90\x83ah\x87V[\x91P[\x83\x15aL\x16W_b\x0FB@\x85\x83`\xC0\x01Q\x84`\x80\x01Q\x01\x02\x81aK\xFFWaK\xFFajPV[`\xC0\x84\x01\x80Q\x92\x90\x91\x04\x91\x82\x90\x03\x90R\x99\x90\x99\x01\x98P[`@\x87\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15aLuWaLi\x81`\xC0\x01Qp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89`@\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x02\x04\x90V[`\xE0\x82\x01\x80Q\x90\x91\x01\x90R[\x80``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aM\x8BW\x80`@\x01Q\x15aMfW_\x80\x86aL\xCFW\x8D`\x01\x01T\x83`\xE0\x01QaL\xDAV[\x82`\xE0\x01Q\x8E`\x02\x01T[\x91P\x91P_aM2\x8F\x85` \x01Q\x85\x85`\x02\x92\x83\x0B_\x90\x81R`\x04\x90\x94\x01` R`@\x90\x93 `\x01\x81\x01\x80T\x90\x92\x03\x90\x91U\x90\x81\x01\x80T\x90\x92\x03\x90\x91UTp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\x0F\x0B\x90V[\x90P\x87\x15aM=W_\x03[aMK\x8A`@\x01Q\x82aB\x86V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x8B\x01RPPP[\x84aMuW\x80` \x01QaM~V[`\x01\x81` \x01Q\x03[`\x02\x0B` \x88\x01RaI\xF7V[\x80Q\x87Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14aM\xB9W\x86QaM~\x90aC\x85V[aI\xF7V[\x86Q` \x88\x01QaNS\x91\x90aN\x15\x90\x89\x90`\xA0\x1Bv\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x90V[\x8CU`@\x87\x01Q`\x03\x8D\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14aN\xC2W`@\x87\x01Q`\x03\x8D\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[\x84aN\xD6W`\xE0\x81\x01Q`\x02\x8D\x01UaN\xE1V[`\xE0\x81\x01Q`\x01\x8D\x01U[\x8AQ_\x13\x85\x15\x15\x14aO\x0EWaO\x07aN\xF9\x83a&~V[a)%\x85\x8E_\x01Q\x03a&~V[\x99PaO+V[aO(aO\x1F\x84\x8D_\x01Q\x03a&~V[a)%\x84a&~V[\x99P[PPPPPP[\x92\x95\x91\x94P\x92PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x92\x90aOz\x90\x84\x90aa\x8DV[\x90\x91UPP`@\x80Q3\x81R` \x81\x01\x83\x90R\x83\x91_\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x91\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01a AV[`@Q\x83\x81R\x82`\x02\x0B`\x04\x82\x01R\x81`\x02\x0B`$\x82\x01R`D\x81\xFD[_\x83\x83\x02\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80\x84\x11aP,W_\x80\xFD[\x80_\x03aP>WP\x82\x90\x04\x90Pa\x088V[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aP\xE2W\x92\x93\x92[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16aQ\tWb\xBF\xC9!_R`\x04`\x1C\xFD[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x90\x1B\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x86\x03\x16\x83aQ\x95W\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aQ\x82\x83\x83\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aO\xEEV[\x81aQ\x8FWaQ\x8FajPV[\x04a0\x9DV[a0\x9DaQ\xB9\x83\x83\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aU\xB6V[\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x82\x04\x91\x06\x15\x15\x01\x90V[\x80_\x81\x12\x15a\x0CbWa\x0Cb\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16\x90\x86\x16\x03`\xFF\x81\x90\x1D\x90\x81\x01\x18l\x01\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16aR_\x81\x84\x84aO\xEEV[\x93P\x84_\x83\x85\x84\t\x11\x16\x84\x01\x93PPPP\x94\x93PPPPV[_\x80\x82\x11aR\x84W_\x80\xFD[P\x7F\x07\x06\x06\x05\x06\x02\x05\0\x06\x02\x03\x02\x05\x04\0\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBEo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11`\x07\x1B\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[b\xBF\xFF\xFF\x81\x16a\x0Cb\x81a$%V[_\x80`\x02\x84\x81\x0B\x90\x86\x90\x0B\x81\x81\x07\x83\x13\x91\x90\x05\x03\x83\x15aS\xB9W`\x02\x81\x90\x0B`\x08\x1D`\x01\x81\x90\x0B_\x90\x81R` \x89\x90R`@\x90 T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xFF\x80\x85\x16\x90\x81\x90\x03\x91\x90\x91\x1C\x91\x82\x16\x80\x15\x15\x95P\x90\x91\x90\x85aS\x9BW\x88\x83`\xFF\x16\x86\x03\x02aS\xAEV[\x88aS\xA5\x82aRxV[\x84\x03`\xFF\x16\x86\x03\x02[\x96PPPPPaT<V[`\x01\x90\x81\x01`\x02\x81\x90\x0B`\x08\x1D\x80\x83\x0B_\x90\x81R` \x8A\x90R`@\x90 T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xFF\x84\x16\x94\x85\x1B\x01\x19\x90\x81\x16\x80\x15\x15\x95P\x92\x93\x91\x92\x85aT\"W\x88\x83`\xFF\x03`\xFF\x16\x86\x01\x02aT5V[\x88\x83aT-\x83aU\xE6V[\x03`\xFF\x16\x86\x01\x02[\x96PPPPP[P\x94P\x94\x92PPPV[_\x80\x80\x80b\xFF\xFF\xFF\x85\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8A\x16\x90\x8B\x16\x10\x15\x82\x88\x12\x80\x15aU)W_aT\x8C\x8A_\x03\x85b\x0FB@\x03b\x0FB@aO\xEEV[\x90P\x82aT\xA5WaT\xA0\x8D\x8D\x8D`\x01aR\rV[aT\xB2V[aT\xB2\x8C\x8E\x8D`\x01aP\xA9V[\x96P\x86\x81\x10aT\xE6W\x8B\x97Pb\x0FB@\x84\x14aT\xDDWaT\xD8\x87\x85\x86b\x0FB@\x03aU\xB6V[aT\xDFV[\x86[\x94PaT\xFFV[\x80\x96PaT\xF5\x8D\x8C\x83\x86aV\x80V[\x97P\x86\x8A_\x03\x03\x94P[\x82aU\x15WaU\x10\x8D\x89\x8D_aP\xA9V[aU!V[aU!\x88\x8E\x8D_aR\rV[\x95PPaU\xA7V[\x81aU?WaU:\x8C\x8C\x8C_aP\xA9V[aUKV[aUK\x8B\x8D\x8C_aR\rV[\x94P\x84\x89\x10aU\\W\x8A\x96PaUnV[\x88\x94PaUk\x8C\x8B\x87\x85aV\xE4V[\x96P[\x81aU\x85WaU\x80\x8C\x88\x8C`\x01aR\rV[aU\x92V[aU\x92\x87\x8D\x8C`\x01aP\xA9V[\x95PaU\xA4\x86\x84\x85b\x0FB@\x03aU\xB6V[\x93P[PPP\x95P\x95P\x95P\x95\x91PPV[_aU\xC2\x84\x84\x84aO\xEEV[\x90P\x81\x80aU\xD2WaU\xD2ajPV[\x83\x85\t\x15a\x088W`\x01\x01\x80a\x088W_\x80\xFD[_\x80\x82\x11aU\xF2W_\x80\xFD[P~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05_\x82\x90\x03\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x15\x17\x15aV\xC0WcO$a\xB8_R`\x04`\x1C\xFD[\x81aV\xD7WaV\xD2\x85\x85\x85`\x01aW=V[a+\"V[a+\"\x85\x85\x85`\x01aX\x9FV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x15\x17\x15aW$WcO$a\xB8_R`\x04`\x1C\xFD[\x81aW5WaV\xD2\x85\x85\x85_aX\x9FV[a+\"\x85\x85\x85_[_\x81\x15aW\xE2W_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15aW\x90WaW\x8B\x84l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x87o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aO\xEEV[aW\xB0V[aW\xB0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16``\x86\x90\x1Baj}V[\x90PaW\xDAaW\xD5\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16aa\xA0V[aY\xD4V[\x91PPa\x1D\x16V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15aX.WaX)\x84l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x87o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aU\xB6V[aXTV[aXT``\x85\x90\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x80\x82\x04\x91\x06\x15\x15\x01\x90V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x11aX\x80WcC#\xA5U_R`\x04`\x1C\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x03\x90Pa\x1D\x16V[_\x82_\x03aX\xAEWP\x83a\x1D\x16V[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x85\x90\x1B\x16\x82\x15aYyWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84\x81\x02\x90\x85\x82\x81aY\x01WaY\x01ajPV[\x04\x03aY>W\x81\x81\x01\x82\x81\x10aY<WaY2\x83\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83aU\xB6V[\x93PPPPa\x1D\x16V[P[PaW\xDA\x81\x85aYds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x83aj}V[aYn\x91\x90aa\xA0V[\x80\x82\x04\x91\x06\x15\x15\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84\x81\x02\x90\x85\x82\x04\x14\x81\x83\x11\x16aY\xADWc\xF5\xC7\x87\xF1_R`\x04`\x1C\xFD[\x80\x82\x03aY2aW\xD5\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x84aU\xB6V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0CbWa\x0Cb\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x17\x99W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15aZMW_\x80\xFD[\x825aZX\x81aZ\x1BV[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15aZvW_\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x088W_\x80\xFD[_\x80_``\x84\x86\x03\x12\x15aZ\xB7W_\x80\xFD[\x835aZ\xC2\x81aZ\x1BV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[_\x80_``\x84\x86\x03\x12\x15aZ\xE9W_\x80\xFD[\x835aZ\xF4\x81aZ\x1BV[\x92P` \x84\x015a[\x04\x81aZ\x1BV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a[%W_\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a[|Wa[|a[,V[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a[\xC9Wa[\xC9a[,V[`@R\x91\x90PV[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\x0CbW_\x80\xFD[\x805`\x02\x81\x90\x0B\x81\x14a\x0CbW_\x80\xFD[_`\xA0\x82\x84\x03\x12\x15a\\\x04W_\x80\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\\'Wa\\'a[,V[`@R\x90P\x80\x825a\\8\x81aZ\x1BV[\x81R` \x83\x015a\\H\x81aZ\x1BV[` \x82\x01Ra\\Y`@\x84\x01a[\xD1V[`@\x82\x01Ra\\j``\x84\x01a[\xE3V[``\x82\x01R`\x80\x83\x015a\\}\x81aZ\x1BV[`\x80\x91\x90\x91\x01R\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12a\\\x9AW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\xB1W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\\\xC8W_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_a\x01\0\x86\x88\x03\x12\x15a\\\xE4W_\x80\xFD[a\\\xEE\x87\x87a[\xF4V[\x94P`\xA0\x86\x015\x93P`\xC0\x86\x015\x92P`\xE0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\x17W_\x80\xFD[a]#\x88\x82\x89\x01a\\\x8AV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_` \x82\x84\x03\x12\x15a]DW_\x80\xFD[\x815a\x088\x81aZ\x1BV[_\x80`@\x83\x85\x03\x12\x15a]`W_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a]\xA6W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a]\x88V[P\x90\x95\x94PPPPPV[_\x80` \x83\x85\x03\x12\x15a]\xC2W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\xD8W_\x80\xFD[a]\xE4\x85\x82\x86\x01a\\\x8AV[\x90\x96\x90\x95P\x93PPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_\x80`\xC0\x83\x85\x03\x12\x15a^TW_\x80\xFD[a^^\x84\x84a[\xF4V[\x91Pa^l`\xA0\x84\x01a[\xD1V[\x90P\x92P\x92\x90PV[\x805\x80\x15\x15\x81\x14a\x0CbW_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a^\x95W_\x80\xFD[\x825a^\xA0\x81aZ\x1BV[\x91Pa^l` \x84\x01a^uV[_\x80_\x80\x84\x86\x03a\x01@\x81\x12\x15a^\xC3W_\x80\xFD[a^\xCD\x87\x87a[\xF4V[\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x82\x01\x12\x15a^\xFEW_\x80\xFD[Pa_\x07a[YV[a_\x13`\xA0\x87\x01a[\xE3V[\x81Ra_!`\xC0\x87\x01a[\xE3V[` \x82\x01R`\xE0\x86\x015`@\x82\x01Ra\x01\0\x86\x015``\x82\x01R\x92Pa\x01 \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a_WW_\x80\xFD[a_c\x87\x82\x88\x01a\\\x8AV[\x95\x98\x94\x97P\x95PPPPV[_\x80_\x80`\xE0\x85\x87\x03\x12\x15a_\x82W_\x80\xFD[a_\x8C\x86\x86a[\xF4V[\x93P`\xA0\x85\x015a_\x9C\x81aZ\x1BV[\x92P`\xC0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a_WW_\x80\xFD[_\x80` \x83\x85\x03\x12\x15a_\xC8W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a_\xDEW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a_\xEEW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a`\x04W_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a`\x18W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a`9W_\x80\xFD[\x825a`D\x81aZ\x1BV[\x91P` \x83\x015a`T\x81aZ\x1BV[\x80\x91PP\x92P\x92\x90PV[_\x80_\x80\x84\x86\x03a\x01 \x81\x12\x15a`tW_\x80\xFD[a`~\x87\x87a[\xF4V[\x94P``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x82\x01\x12\x15a`\xAFW_\x80\xFD[P`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a`\xD3Wa`\xD3a[,V[`@Ra`\xE2`\xA0\x87\x01a^uV[\x81R`\xC0\x86\x015` \x82\x01R`\xE0\x86\x015a`\xFC\x81aZ\x1BV[`@\x82\x01R\x92Pa\x01\0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a_WW_\x80\xFD[_\x80_\x80`\x80\x85\x87\x03\x12\x15aa0W_\x80\xFD[\x845aa;\x81aZ\x1BV[\x93P` \x85\x015aaK\x81aZ\x1BV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07eWa\x07eaa`V[\x80\x82\x01\x80\x82\x11\x15a\x07eWa\x07eaa`V[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[` \x81R_a\x1D\x16` \x83\x01\x84\x86aa\xB3V[_` \x82\x84\x03\x12\x15ab\x1DW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ab3W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13abCW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ab]Wab]a[,V[ab\x8E` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a[\x82V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15ab\xA2W_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81RacZ` \x82\x01\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x84`\xC0\x82\x01R\x83`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01R_ac\x7Fa\x01 \x83\x01\x84\x86aa\xB3V[\x98\x97PPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81Rad&` \x82\x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x83Q`\x02\x90\x81\x0B`\xC0\x83\x01R` \x85\x01Q\x90\x0B`\xE0\x82\x01R`@\x84\x01Qa\x01\0\x82\x01R``\x84\x01Qa\x01 \x82\x01Ra\x01`a\x01@\x82\x01R_a0\x9Da\x01`\x83\x01\x84\x86aa\xB3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x81Rae\x08` \x82\x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x85Q`\x02\x90\x81\x0B`\xC0\x83\x01R` \x87\x01Q\x90\x0B`\xE0\x82\x01R`@\x86\x01Qa\x01\0\x82\x01R``\x86\x01Qa\x01 \x82\x01R\x84a\x01@\x82\x01R\x83a\x01`\x82\x01Ra\x01\xA0a\x01\x80\x82\x01R_aF\x9Ca\x01\xA0\x83\x01\x84\x86aa\xB3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81Rae\xF8` \x82\x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\xC0\x82\x01Ra\x01\0`\xE0\x82\x01R_a0\x9Da\x01\0\x83\x01\x84\x86aa\xB3V[`\xA0\x81\x01a\x07e\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81RagJ` \x82\x01\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\xC0\x82\x01R\x83`\x02\x0B`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01R_ac\x7Fa\x01 \x83\x01\x84\x86aa\xB3V[_` \x82\x84\x03\x12\x15ag\x98W_\x80\xFD[PQ\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81Rah:` \x82\x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x83Q\x15\x15`\xC0\x82\x01R` \x84\x01Q`\xE0\x82\x01R`@\x84\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x82\x01Ra\x01@a\x01 \x82\x01R_a0\x9Da\x01@\x83\x01\x84\x86aa\xB3V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15ah\xA6Wah\xA6aa`V[PP\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81RaiI` \x82\x01\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x84Q\x15\x15`\xC0\x82\x01R` \x85\x01Q`\xE0\x82\x01R`@\x85\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x82\x01R\x83a\x01 \x82\x01Ra\x01`a\x01@\x82\x01R_ac\x7Fa\x01`\x83\x01\x84\x86aa\xB3V[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x12\x17\x15a\x07eWa\x07eaa`V[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x12o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x13\x17\x15a\x07eWa\x07eaa`V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07eWa\x07eaa`V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82aj\xB0W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a;7Wa;7aa`V\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    /// The runtime bytecode of the contract, as deployed on the network.
    ///
    /// ```text
    ///0x6080604052600436106101f4575f3560e01c80635a6bcfda11610117578063a5841194116100ac578063f135baaa1161007c578063f3cd914c11610062578063f3cd914c14610676578063f5298aca14610695578063fe99049a146106b4575f80fd5b8063f135baaa14610638578063f2fde38b14610657575f80fd5b8063a584119414610595578063b6363cf2146105b4578063dbd035ff146105ed578063f02de3b21461060c575f80fd5b80638161b874116100e75780638161b874146104dc5780638da5cb5b146104fb57806397e8cd4e1461054b5780639bf6645f14610576575f80fd5b80635a6bcfda14610438578063695c5bf51461046c5780637e87ce7d1461049e57806380f0b44c146104bd575f80fd5b80632d7713891161018d57806348c894911161015d57806348c894911461039257806352759651146103be578063558a7297146103dd578063598af9e7146103fc575f80fd5b80632d7713891461031557806335fd631a146103345780633dd45adb14610360578063426a849314610373575f80fd5b806311da60b4116101c857806311da60b4146102b0578063156e29f6146102b85780631e2eaeaf146102d7578063234266d7146102f6575f80fd5b8062fdd58e146101f857806301ffc9a714610241578063095bcdb6146102705780630b0d9c091461028f575b5f80fd5b348015610203575f80fd5b5061022e610212366004615a3c565b600460209081525f928352604080842090915290825290205481565b6040519081526020015b60405180910390f35b34801561024c575f80fd5b5061026061025b366004615a66565b6106d3565b6040519015158152602001610238565b34801561027b575f80fd5b5061026061028a366004615aa5565b61076b565b34801561029a575f80fd5b506102ae6102a9366004615ad7565b61083f565b005b61022e6108c9565b3480156102c3575f80fd5b506102ae6102d2366004615aa5565b610927565b3480156102e2575f80fd5b5061022e6102f1366004615b15565b6109ab565b348015610301575f80fd5b5061022e610310366004615ccf565b6109b5565b348015610320575f80fd5b506102ae61032f366004615d34565b610ad9565b34801561033f575f80fd5b5061035361034e366004615d4f565b610bcc565b6040516102389190615d6f565b61022e61036e366004615d34565b610c09565b34801561037e575f80fd5b5061026061038d366004615aa5565b610c67565b34801561039d575f80fd5b506103b16103ac366004615db1565b610cd8565b6040516102389190615df0565b3480156103c9575f80fd5b506102ae6103d8366004615e43565b610e2a565b3480156103e8575f80fd5b506102606103f7366004615e84565b610ecc565b348015610407575f80fd5b5061022e610416366004615ad7565b600560209081525f938452604080852082529284528284209052825290205481565b348015610443575f80fd5b50610457610452366004615eae565b610f66565b60408051928352602083019190915201610238565b348015610477575f80fd5b5061048b610486366004615f6f565b611165565b60405160029190910b8152602001610238565b3480156104a9575f80fd5b506102ae6104b8366004615e43565b611412565b3480156104c8575f80fd5b506102ae6104d7366004615a3c565b611503565b3480156104e7575f80fd5b5061022e6104f6366004615ad7565b6115c2565b348015610506575f80fd5b505f546105269073ffffffffffffffffffffffffffffffffffffffff1681565b60405173ffffffffffffffffffffffffffffffffffffffff9091168152602001610238565b348015610556575f80fd5b5061022e610565366004615d34565b60016020525f908152604090205481565b348015610581575f80fd5b50610353610590366004615fb7565b6116ee565b3480156105a0575f80fd5b506102ae6105af366004615d34565b611727565b3480156105bf575f80fd5b506102606105ce366004616028565b600360209081525f928352604080842090915290825290205460ff1681565b3480156105f8575f80fd5b50610353610607366004615fb7565b6117cc565b348015610617575f80fd5b506002546105269073ffffffffffffffffffffffffffffffffffffffff1681565b348015610643575f80fd5b5061022e610652366004615b15565b611803565b348015610662575f80fd5b506102ae610671366004615d34565b61180d565b348015610681575f80fd5b5061022e61069036600461605f565b6118fc565b3480156106a0575f80fd5b506102ae6106af366004615aa5565b611aae565b3480156106bf575f80fd5b506102606106ce36600461611d565b611b32565b5f7f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316148061076557507f0f632fb3000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316145b92915050565b335f90815260046020908152604080832085845290915281208054839190839061079690849061618d565b909155505073ffffffffffffffffffffffffffffffffffffffff84165f908152600460209081526040808320868452909152812080548492906107da9084906161a0565b9091555050604080513380825260208201859052859273ffffffffffffffffffffffffffffffffffffffff8816927f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac72885991015b60405180910390a45060015b9392505050565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c61088e5761088e7f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b6108a38361089b83611d26565b5f0333611d6b565b6108c473ffffffffffffffffffffffffffffffffffffffff84168383611dcb565b505050565b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610919576109197f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b61092233611ec6565b905090565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610976576109767f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b816109848161089b84611d26565b6109a58473ffffffffffffffffffffffffffffffffffffffff831684611faf565b50505050565b5f81545f5260205ff35b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610a0557610a057f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b610a0d61204e565b60a086205f818152600660205260409020610a27816120b6565b6080880151610a509073ffffffffffffffffffffffffffffffffffffffff1689898989896120fd565b610a5b8188886121e0565b9250610a688884336122da565b6040805188815260208101889052339184917f29ef05caaff9404b7cb6d1c0e9bbae9eaa7ab2541feba1a9c4248594c08156cb910160405180910390a36080880151610ace9073ffffffffffffffffffffffffffffffffffffffff168989898989612302565b505095945050505050565b5f5473ffffffffffffffffffffffffffffffffffffffff163314610b5e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f554e415554484f52495a4544000000000000000000000000000000000000000060448201526064015b60405180910390fd5b600280547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff83169081179091556040517fb4bd8ef53df690b9943d3318996006dbb82a25f54719d8c8035b516a2a5b8acc905f90a250565b6060604051808360051b6020835284602084015260408301925080830190505b85548352602083019250600186019550808310610bec5781810382f35b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610c5957610c597f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b61076582611ec6565b919050565b335f81815260056020908152604080832073ffffffffffffffffffffffffffffffffffffffff881680855290835281842087855290925280832085905551919285927fb3fd5071835887567a0671151121894ddccc2842f1d10bedad13e0d17cace9a79061082c9087815260200190565b60607fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c15610d2a57610d2a7f5090d6c600000000000000000000000000000000000000000000000000000000611d1e565b610d326123da565b6040517f91dd734600000000000000000000000000000000000000000000000000000000815233906391dd734690610d7090869086906004016161fa565b5f604051808303815f875af1158015610d8b573d5f803e3d5ffd5b505050506040513d5f823e601f3d9081017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0168201604052610dd0919081019061620d565b90507f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5c15610e2257610e227f5212cba100000000000000000000000000000000000000000000000000000000611d1e565b610765612400565b604082015162ffffff1662800000141580610e755750816080015173ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614155b15610ea357610ea37f30d2164100000000000000000000000000000000000000000000000000000000611d1e565b610eb18162ffffff16612425565b60a082205f8181526006602052604090206108c49083612464565b335f81815260036020908152604080832073ffffffffffffffffffffffffffffffffffffffff871680855290835281842080547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001687151590811790915591519182529293917fceb576d9f15e4e200fdb5096d64d5dfd667e16def20c1eefd14256d8e3faa267910160405180910390a350600192915050565b5f807fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c610fb757610fb77f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b610fbf61204e565b60a086205f818152600660205260409020610fd9816120b6565b60808801516110019073ffffffffffffffffffffffffffffffffffffffff16898989896124bd565b5f6110756040518060c001604052803373ffffffffffffffffffffffffffffffffffffffff1681526020018a5f015160020b81526020018a6020015160020b81526020016110528b6040015161267e565b600f0b81526060808d015160020b60208301528b015160409091015283906126b3565b945090506110838185612af8565b945050503373ffffffffffffffffffffffffffffffffffffffff16817ff208f4912782fd25c7f114ca3723a2d5dd6f3bcc3ac8db5af63baa85f711d5ec885f015189602001518a604001518b606001516040516111019493929190600294850b81529290930b60208301526040820152606081019190915260800190565b60405180910390a360808701515f906111359073ffffffffffffffffffffffffffffffffffffffff16898987878b8b612b2b565b9094509050801561114f5761114f88828a608001516122da565b61115a8885336122da565b505094509492505050565b5f61116e61204e565b6060850151617fff60029190910b13156111b25760608501516111b2907fb70024f80000000000000000000000000000000000000000000000000000000090612d02565b600160020b856060015160020b12156111f55760608501516111f5907fe9e905880000000000000000000000000000000000000000000000000000000090612d02565b8451602086015173ffffffffffffffffffffffffffffffffffffffff90811691161061124d578451602086015161124d917f6e6c98300000000000000000000000000000000000000000000000000000000091612d11565b61127e8560400151866080015173ffffffffffffffffffffffffffffffffffffffff16612d5490919063ffffffff16565b6112b25760808501516112b2907fe65af6a00000000000000000000000000000000000000000000000000000000090612e22565b5f6112c5866040015162ffffff16612e44565b60808701519091506112f09073ffffffffffffffffffffffffffffffffffffffff1687878787612e69565b60a086205f6112fe88612f40565b5f83815260066020526040902090915061131a908883866130a8565b60808901519094506113469073ffffffffffffffffffffffffffffffffffffffff168989878a8a613183565b876020015173ffffffffffffffffffffffffffffffffffffffff16885f015173ffffffffffffffffffffffffffffffffffffffff16837fdd466e674ea557f56295e2d0218a125ea4b4f0f6f3307b95f85e6110838d64388b604001518c606001518d608001518d8b6040516113ff95949392919062ffffff959095168552600293840b602086015273ffffffffffffffffffffffffffffffffffffffff928316604086015291166060840152900b608082015260a00190565b60405180910390a4505050949350505050565b60025473ffffffffffffffffffffffffffffffffffffffff16331461145a5761145a7f48f5c3ed00000000000000000000000000000000000000000000000000000000611d1e565b6103e9610fff821610623e900062fff000831610166114a2576114a27fa7abe2f70000000000000000000000000000000000000000000000000000000062ffffff8316612e22565b60a082206114c4826114be835f90815260066020526040902090565b9061325c565b60405162ffffff8316815281907fe9c42593e71f84403b84352cd168d693e2c9fcd1fdbcc3feb21d92b43e6696f99060200160405180910390a2505050565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c611552576115527f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b335f90815273ffffffffffffffffffffffffffffffffffffffff8316602052604081205c9061158083611d26565b90508181600f0b146115b5576115b57fbda73abf00000000000000000000000000000000000000000000000000000000611d1e565b6109a584825f0333611d6b565b6002545f9073ffffffffffffffffffffffffffffffffffffffff16331461160c5761160c7f48f5c3ed00000000000000000000000000000000000000000000000000000000611d1e565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c1561165c5761165c7f3e5f4fd600000000000000000000000000000000000000000000000000000000611d1e565b8115611668578161168e565b73ffffffffffffffffffffffffffffffffffffffff83165f908152600160205260409020545b73ffffffffffffffffffffffffffffffffffffffff84165f908152600160205260408120805492935083929091906116c790849061618d565b90915550610838905073ffffffffffffffffffffffffffffffffffffffff84168583611dcb565b606060405180602082528360208301526040820191508360051b8201855b80355c84526020938401930181841061170c575b5081810382f35b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c611776576117767f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b73ffffffffffffffffffffffffffffffffffffffff811661179c576117996132b0565b50565b5f6117bc8273ffffffffffffffffffffffffffffffffffffffff166132d5565b90506117c88282613384565b5050565b606060405180602082528360208301526040820191508360051b8201855b80355484526020938401930181841015611720576117ea565b5f815c5f5260205ff35b5f5473ffffffffffffffffffffffffffffffffffffffff16331461188d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600c60248201527f554e415554484f52495a454400000000000000000000000000000000000000006044820152606401610b55565b5f80547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff83169081178255604051909133917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a350565b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c61194c5761194c7f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b61195461204e565b83602001515f03611988576119887fbe8b850700000000000000000000000000000000000000000000000000000000611d1e565b60a085205f8181526006602052604090206119a2816120b6565b60808701515f90819081906119d09073ffffffffffffffffffffffffffffffffffffffff168b8b8b8b6133e4565b809350819550829450505050611a4c84866040518060a001604052808681526020018e6060015160020b81526020018d5f0151151581526020018d6040015173ffffffffffffffffffffffffffffffffffffffff1681526020018562ffffff168152508c5f0151611a45578d6020015161358b565b8d5161358b565b60808b01519096505f9250611a7d915073ffffffffffffffffffffffffffffffffffffffff168a8a888b8b8861368c565b90955090508015611a9757611a9789828b608001516122da565b611aa28986336122da565b50505050949350505050565b7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235c611afd57611afd7f54e3ca0d00000000000000000000000000000000000000000000000000000000611d1e565b81611b1181611b0b84611d26565b33611d6b565b6109a58473ffffffffffffffffffffffffffffffffffffffff831684613825565b5f3373ffffffffffffffffffffffffffffffffffffffff861614801590611b89575073ffffffffffffffffffffffffffffffffffffffff85165f90815260036020908152604080832033845290915290205460ff16155b15611c325773ffffffffffffffffffffffffffffffffffffffff85165f90815260056020908152604080832033845282528083208684529091529020547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8114611c3057611bf7838261618d565b73ffffffffffffffffffffffffffffffffffffffff87165f90815260056020908152604080832033845282528083208884529091529020555b505b73ffffffffffffffffffffffffffffffffffffffff85165f90815260046020908152604080832086845290915281208054849290611c7190849061618d565b909155505073ffffffffffffffffffffffffffffffffffffffff84165f90815260046020908152604080832086845290915281208054849290611cb59084906161a0565b90915550506040805133815260208101849052849173ffffffffffffffffffffffffffffffffffffffff80881692908916917f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac728859910160405180910390a45060015b949350505050565b805f5260045ffd5b5f6f800000000000000000000000000000008210611d6757611d677f93dafdf100000000000000000000000000000000000000000000000000000000611d1e565b5090565b81600f0b5f03611d7a57505050565b5f80611d9d73ffffffffffffffffffffffffffffffffffffffff86168486613936565b91509150805f03611db557611db061397c565b611dc4565b815f03611dc457611dc46139ca565b5050505050565b5f73ffffffffffffffffffffffffffffffffffffffff8416611e25575f805f8085875af1905080611e2057611e207f8549db590000000000000000000000000000000000000000000000000000000084613a18565b6109a5565b6040517fa9059cbb00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8416600482015282602482015260205f6044835f895af13d15601f3d1160015f511416171691505f81525f60208201525f604082015250806109a5576109a57fb12c5f9c0000000000000000000000000000000000000000000000000000000085613a18565b5f7f27e098c505d44ec3574004bca052aabf76bd35004c182099d8c575fb238593b95c73ffffffffffffffffffffffffffffffffffffffff8116611f0c57349150611f96565b3415611f3b57611f3b7fb0ec849e00000000000000000000000000000000000000000000000000000000611d1e565b7f1e0745a7db1623981f0b2a5d4232364c00787266eb75ad546f190e6cebe9bd955c5f611f7d73ffffffffffffffffffffffffffffffffffffffff84166132d5565b9050611f89828261618d565b9350611f936132b0565b50505b611fa981611fa384611d26565b85611d6b565b50919050565b73ffffffffffffffffffffffffffffffffffffffff83165f90815260046020908152604080832085845290915281208054839290611fee9084906161a0565b90915550506040805133815260208101839052839173ffffffffffffffffffffffffffffffffffffffff8616915f917f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac72885991015b60405180910390a4505050565b3073ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016146120b4576120b47f0d89438e00000000000000000000000000000000000000000000000000000000611d1e565b565b805473ffffffffffffffffffffffffffffffffffffffff165f03611799576117997f486aa30700000000000000000000000000000000000000000000000000000000611d1e565b853373ffffffffffffffffffffffffffffffffffffffff8216146121d75760208716156121d7576121d5338787878787604051602401612142969594939291906162bf565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fb6a8b0fa0000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff891690613a4b565b505b50505050505050565b60038301545f906fffffffffffffffffffffffffffffffff16808203612229576122297fa74f97ab00000000000000000000000000000000000000000000000000000000611d1e565b61226061223585611d26565b5f0361224085611d26565b5f0360809190911b6fffffffffffffffffffffffffffffffff9091161790565b9150831561229a576001850180546fffffffffffffffffffffffffffffffff83167001000000000000000000000000000000008702040190555b82156122d2576002850180546fffffffffffffffffffffffffffffffff83167001000000000000000000000000000000008602040190555b509392505050565b82516122f0906122ea8460801d90565b83611d6b565b6108c483602001516122ea84600f0b90565b853373ffffffffffffffffffffffffffffffffffffffff8216146121d75760108716156121d7576121d5338787878787604051602401612347969594939291906162bf565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fe1b4af690000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff891690613a4b565b60017fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235d565b5f7fc090fc4683624cfc3884e9d8de5eca132f2d0ec062aff75d43c0465d5ceeab235d565b620f424062ffffff82161115611799576117997f140021130000000000000000000000000000000000000000000000000000000062ffffff8316612e22565b61246d826120b6565b81547fffffff000000ffffffffffffffffffffffffffffffffffffffffffffffffffff167cffffff000000000000000000000000000000000000000000000000000060d083901b16175b90915550565b843373ffffffffffffffffffffffffffffffffffffffff821614612676575f84604001511380156124f15750610800861615155b156125ab576125a5338686868660405160240161251295949392919061638b565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f259982e50000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff881690613a4b565b50612676565b5f8460400151131580156125c25750610200861615155b15612676576121d733868686866040516024016125e395949392919061638b565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f21d0ee700000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff881690613a4b565b505050505050565b80600f81900b8114610c6257610c627f93dafdf100000000000000000000000000000000000000000000000000000000611d1e565b6060810151602082015160408301515f92839290916126d28282613b3e565b604080516080810182525f80825260208201819052918101829052606081019190915283600f0b5f146128a25761270b8884865f613c05565b6fffffffffffffffffffffffffffffffff166020830152151581526127338883866001613c05565b6fffffffffffffffffffffffffffffffff166060830152151560408201525f600f85900b126128675760808701515f9060020b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2761881810783139082900503620d89e891909105036001016fffffffffffffffffffffffffffffffff049050806fffffffffffffffffffffffffffffffff1682602001516fffffffffffffffffffffffffffffffff16111561280b5761280b7fb8e3c3850000000000000000000000000000000000000000000000000000000085612d02565b806fffffffffffffffffffffffffffffffff1682606001516fffffffffffffffffffffffffffffffff161115612865576128657fb8e3c3850000000000000000000000000000000000000000000000000000000084612d02565b505b8051156128835760808701516128839060058a01908590613cee565b8060400151156128a25760808701516128a29060058a01908490613cee565b5f806128af8a8686613d40565b8a5160a08c015160408051602681019290925260068083018a9052600383018b9052928252603a600c8301205f838301819052602080850182905293819052908152928f0190915281209294509092508061290c838a8787613df4565b9150915061294161291c83611d26565b61292583611d26565b6fffffffffffffffffffffffffffffffff1660809190911b1790565b995050505050505f84600f0b12156129aa5780511561297d57600283810b5f90815260048a016020526040812081815560018101829055909101555b8060400151156129aa57600282810b5f90815260048a016020526040812081815560018101829055909101555b5082600f0b5f14612aee5786545f806129c68360a01c60020b90565b73ffffffffffffffffffffffffffffffffffffffff8416915091508460020b8260020b1215612a2257612a1b612a15612a10612a0188613f25565b612a0a88613f25565b8a614222565b61267e565b60801b90565b9750612aea565b8360020b8260020b1215612ac557612a59612a43612a1083612a0a88613f25565b612925612a10612a5289613f25565b858b61425a565b60038b0154909850612a7d906fffffffffffffffffffffffffffffffff1687614286565b60038b0180547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166fffffffffffffffffffffffffffffffff92909216919091179055612aea565b612ae75f612925612a10612ad889613f25565b612ae189613f25565b8b61425a565b97505b5050505b5050509250929050565b5f608082811d9084901d01600f83810b9085900b01612b22612b198361267e565b6129258361267e565b95945050505050565b5f8073ffffffffffffffffffffffffffffffffffffffff89163303612b5457508490505f612cf6565b8591505f87604001511315612c3c57610400891615612c3757612c2833898989898989604051602401612b8d979695949392919061646d565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f9f063efc0000000000000000000000000000000000000000000000000000000017905260028b1615155b73ffffffffffffffffffffffffffffffffffffffff8c1691906142b6565b9050612c348282614310565b91505b612cf6565b610100891615612cf657612ce733898989898989604051602401612c66979695949392919061646d565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f6c2bbe7e0000000000000000000000000000000000000000000000000000000017905260018b161515612c0a565b9050612cf38282614310565b91505b97509795505050505050565b815f528060020b60045260245ffd5b60405183815273ffffffffffffffffffffffffffffffffffffffff8316600482015273ffffffffffffffffffffffffffffffffffffffff82166024820152604481fd5b5f60808316158015612d6857506008831615155b15612d7457505f610765565b60408316158015612d8757506004831615155b15612d9357505f610765565b6104008316158015612da757506002831615155b15612db357505f610765565b6101008316158015612dc757506001831615155b15612dd357505f610765565b73ffffffffffffffffffffffffffffffffffffffff831615612e1157613fff8316151580612e0c57506280000062ffffff831614610838565b610838565b5062ffffff16628000001415919050565b815f5273ffffffffffffffffffffffffffffffffffffffff811660045260245ffd5b5f6280000062ffffff831603612e5b57505f919050565b611d678262ffffff16612425565b843373ffffffffffffffffffffffffffffffffffffffff82161461267657612000861615612676576121d73386868686604051602401612ead95949392919061655d565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f3440d8200000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff881690613a4b565b6002545f9073ffffffffffffffffffffffffffffffffffffffff1615610c62575f612f6c456064614331565b9050805a1015612f9f57612f9f7f1ee4970200000000000000000000000000000000000000000000000000000000611d1e565b60025460405173ffffffffffffffffffffffffffffffffffffffff909116905f90612fce90869060240161662c565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152919052602080820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f553bfc370000000000000000000000000000000000000000000000000000000017815282519293505f9283929183919082888af15f513d6020149091169250905081801561307457508062ffffff1681145b801561309157506103e9610fff821610623e900062fff000831610165b61309b575f61309d565b805b979650505050505050565b83545f9073ffffffffffffffffffffffffffffffffffffffff16156130f0576130f07f7983c05100000000000000000000000000000000000000000000000000000000611d1e565b6130f984614385565b90507cffffff000000000000000000000000000000000000000000000000000060d083901b1676ffffff000000000000000000000000000000000000000060a083901b1673ffffffffffffffffffffffffffffffffffffffff86161760b885901b79ffffff0000000000000000000000000000000000000000000000161717909455509192915050565b853373ffffffffffffffffffffffffffffffffffffffff8216146121d7576110008716156121d7576121d53387878787876040516024016131c9969594939291906166af565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fa910f80f0000000000000000000000000000000000000000000000000000000017905273ffffffffffffffffffffffffffffffffffffffff891690613a4b565b613265826120b6565b81547fffffffffffff000000ffffffffffffffffffffffffffffffffffffffffffffff1679ffffff000000000000000000000000000000000000000000000060b883901b16176124b7565b5f7f27e098c505d44ec3574004bca052aabf76bd35004c182099d8c575fb238593b95d565b5f73ffffffffffffffffffffffffffffffffffffffff82166132f8575047919050565b6040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff8316906370a0823190602401602060405180830381865afa158015613360573d5f803e3d5ffd5b505050506040513d601f19601f820116820180604052508101906107659190616788565b73ffffffffffffffffffffffffffffffffffffffff82167f27e098c505d44ec3574004bca052aabf76bd35004c182099d8c575fb238593b95d807f1e0745a7db1623981f0b2a5d4232364c00787266eb75ad546f190e6cebe9bd955d5050565b60208301515f8073ffffffffffffffffffffffffffffffffffffffff88163303613410575f9150613580565b6080881615613580575f6134b289338a8a8a8a60405160240161343795949392919061679f565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f575e24b400000000000000000000000000000000000000000000000000000000179052613a4b565b905080516060146134e6576134e67f1e048e1d00000000000000000000000000000000000000000000000000000000611d1e565b604088015162ffffff16628000000361350157606081015191505b600889161561357e57604081015192505f61351c8460801d90565b905080600f0b5f1461357c575f8512613539600f83900b87616887565b955080613548575f861261354c565b5f86135b1561357a5761357a7ffa0b71d600000000000000000000000000000000000000000000000000000000611d1e565b505b505b505b955095509592505050565b5f8080808061359a89886146a9565b93509350935093505f8311156135d55773ffffffffffffffffffffffffffffffffffffffff86165f9081526001602052604090208054840190555b33887f40e9cecb9f5f1f1c5b9c97dec2917b7ee92e57ba5563708daca94dd84ad7112f6136028760801d90565b61360c88600f0b90565b85516040808801516020808a01518351600f97880b81529590960b9085015273ffffffffffffffffffffffffffffffffffffffff909216908301526fffffffffffffffffffffffffffffffff16606082015260029190910b608082015262ffffff861660a082015260c00160405180910390a35091979650505050505050565b5f8073ffffffffffffffffffffffffffffffffffffffff891633036136b557508490505f612cf6565b5f6136c08460801d90565b90505f6136cd85600f0b90565b905060408b16156137a057613793612a10338c8c8c8c8c6040516024016136f9969594939291906168ae565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fb47b2fb10000000000000000000000000000000000000000000000000000000017905260048e16151573ffffffffffffffffffffffffffffffffffffffff8f1691906142b6565b61379d908261699d565b90505b5f81600f0b5f1415806137b6575082600f0b5f14155b1561381357895160208b01515f13901515146137ea576fffffffffffffffffffffffffffffffff8316608083901b17613804565b6fffffffffffffffffffffffffffffffff8216608084901b175b90506138108982614310565b98505b979b979a509698505050505050505050565b3373ffffffffffffffffffffffffffffffffffffffff8416811480159061387e575073ffffffffffffffffffffffffffffffffffffffff8085165f9081526003602090815260408083209385168352929052205460ff16155b1561392b5773ffffffffffffffffffffffffffffffffffffffff8085165f9081526005602090815260408083209385168352928152828220868352905220547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8114613929576138ee838261618d565b73ffffffffffffffffffffffffffffffffffffffff8087165f9081526005602090815260408083209387168352928152828220888352905220555b505b6109a5848484614f3b565b73ffffffffffffffffffffffffffffffffffffffff8281165f90815290841660205260408120805c919061396e600f85900b84616887565b915081815d50935093915050565b7f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5c600181039050807f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5d50565b7f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5c600181019050807f7d4b3164c6e45b97e7d87b7125a44c5828d005af88f9d751cfd78729c5d99a0b5d50565b3d60405183815282600482015260406024820152816044820152815f606483013e602080601f8401040260640191508181fd5b60605f805f8451602086015f885af1905080613a8b57613a8b7f319d54c30000000000000000000000000000000000000000000000000000000085613a18565b6040519150601f19603f3d011682016040523d82523d5f602084013e602082511080613b09575060208301517fffffffff0000000000000000000000000000000000000000000000000000000016613ae4836020015190565b7fffffffff000000000000000000000000000000000000000000000000000000001614155b15613b3757613b377f1e048e1d00000000000000000000000000000000000000000000000000000000611d1e565b5092915050565b8060020b8260020b12613b7657613b767fc4433ed5000000000000000000000000000000000000000000000000000000008383614fd1565b7ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff27618600283900b1215613bcc57613bcc7fd5e2f7ab0000000000000000000000000000000000000000000000000000000083612d02565b620d89e8600282900b13156117c8576117c87f1ad777f80000000000000000000000000000000000000000000000000000000082612d02565b600283900b5f908152600485016020526040812080548291906fffffffffffffffffffffffffffffffff8116907001000000000000000000000000000000009004600f0b613c538288614286565b6fffffffffffffffffffffffffffffffff808216159084168015919091141596509094505f03613ca657885460a01c60020b60020b8860020b13613ca6576001808a0154908401556002808a0154908401555b5f86613cbb57613cb6888361699d565b613cc5565b613cc588836169eb565b90508060801b6fffffffffffffffffffffffffffffffff86161784555050505094509492505050565b600291820b910b80820715613d1b5760405163d4d8f3e681528260208201528160408201526044601c8201fd5b80820591508160081d5f528260205260405f20600160ff84161b815418815550505050565b600282810b5f81815260048601602052604080822085850b83529082208754929485949293919260a09290921c900b90811215613d96578160010154836001015403945081600201548360020154039350613de9565b8560020b8160020b12613dc2578260010154826001015403945082600201548260020154039350613de9565b81600101548360010154896001015403039450816002015483600201548960020154030393505b505050935093915050565b83545f9081906fffffffffffffffffffffffffffffffff16600f86900b8203613e5e57806fffffffffffffffffffffffffffffffff165f03613e5957613e597faefeb92400000000000000000000000000000000000000000000000000000000611d1e565b613ea5565b613e688187614286565b87547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166fffffffffffffffffffffffffffffffff919091161787555b613ed987600101548603826fffffffffffffffffffffffffffffffff16700100000000000000000000000000000000614fee565b9250613f0f87600201548503826fffffffffffffffffffffffffffffffff16700100000000000000000000000000000000614fee565b6001880195909555505060029094015591929050565b60020b5f60ff82901d80830118620d89e8811115613f6757613f677f8b86327a0000000000000000000000000000000000000000000000000000000084612d02565b7001fffcb933bd6fad37aa2d162d1a5940016001821602700100000000000000000000000000000000186002821615613fb0576ffff97272373d413259a46990580e213a0260801c5b6004821615613fcf576ffff2e50f5f656932ef12357cf3c7fdcc0260801c5b6008821615613fee576fffe5caca7e10e4e61c3624eaa0941cd00260801c5b601082161561400d576fffcb9843d60f6159c9db58835c9266440260801c5b602082161561402c576fff973b41fa98c081472e6896dfb254c00260801c5b604082161561404b576fff2ea16466c96a3843ec78b326b528610260801c5b608082161561406a576ffe5dee046a99a2a811c461f1969c30530260801c5b61010082161561408a576ffcbe86c7900a88aedcffc83b479aa3a40260801c5b6102008216156140aa576ff987a7253ac413176f2b074cf7815e540260801c5b6104008216156140ca576ff3392b0822b70005940c7a398e4b70f30260801c5b6108008216156140ea576fe7159475a2c29b7443b29c7fa6e889d90260801c5b61100082161561410a576fd097f3bdfd2022b8845ad8f792aa58250260801c5b61200082161561412a576fa9f746462d870fdf8a65dc1f90e061e50260801c5b61400082161561414a576f70d869a156d2a1b890bb3df62baf32f70260801c5b61800082161561416a576f31be135f97d08fd981231505542fcfa60260801c5b6201000082161561418b576f09aa508b5b7a84e1c677de54f3e99bc90260801c5b620200008216156141ab576e5d6af8dedb81196699c329225ee6040260801c5b620400008216156141ca576d2216e584f5fa1ea926041bedfe980260801c5b620800008216156141e7576b048a170391f7dc42444e8fa20260801c5b5f841315614212577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff045b63ffffffff0160201c9392505050565b5f8082600f0b126142495761424261423d85858560016150a9565b6151db565b5f03611d16565b611d1661423d8585855f035f6150a9565b5f8082600f0b126142755761424261423d858585600161520d565b611d1661423d8585855f035f61520d565b6fffffffffffffffffffffffffffffffff8216600f82900b01608081901c15610765576393dafdf15f526004601cfd5b5f806142c28585613a4b565b9050826142d2575f915050610838565b8051604014614304576143047f1e048e1d00000000000000000000000000000000000000000000000000000000611d1e565b60400151949350505050565b5f608082811d9084901d03600f83810b9085900b03612b22612b198361267e565b5f61271082111561436e576040517fdeaa01e600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61271061437b8385616a39565b6108389190616a7d565b5f73fffd8963efd1fc6a506488495d951d51639616827ffffffffffffffffffffffffffffffffffffffffffffffffffffffffefffd895d830173ffffffffffffffffffffffffffffffffffffffff161115614404576144047f614875240000000000000000000000000000000000000000000000000000000083612e22565b77ffffffffffffffffffffffffffffffffffffffff00000000602083901b16805f61442e82615278565b60ff1690506080811061444957607f810383901c9150614453565b80607f0383901b91505b908002607f81811c60ff83811c9190911c800280831c81831c1c800280841c81841c1c800280851c81851c1c800280861c81861c1c800280871c81871c1c800280881c81881c1c800280891c81891c1c8002808a1c818a1c1c8002808b1c818b1c1c8002808c1c818c1c1c8002808d1c818d1c1c8002808e1c9c81901c9c909c1c80029c8d901c9e9d7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff808f0160401b60c09190911c678000000000000000161760c19b909b1c674000000000000000169a909a1760c29990991c672000000000000000169890981760c39790971c671000000000000000169690961760c49590951c670800000000000000169490941760c59390931c670400000000000000169290921760c69190911c670200000000000000161760c79190911c670100000000000000161760c89190911c6680000000000000161760c99190911c6640000000000000161760ca9190911c6620000000000000161760cb9190911c6610000000000000161760cc9190911c6608000000000000161760cd9190911c66040000000000001617693627a301d71055774c8581027ffffffffffffffffffffffffffffffffffd709b7e5480fba5a50fed5e62ffc5568101608090811d906fdb2df09e81959a81455e260799a0632f8301901d600281810b9083900b1461469a578873ffffffffffffffffffffffffffffffffffffffff1661467282613f25565b73ffffffffffffffffffffffffffffffffffffffff161115614694578161469c565b8061469c565b815b9998505050505050505050565b604080516060810182525f8082526020820181905291810182905281908190855460408601515f816146e357610fff60c484901c166146ed565b610fff60b884901c165b885173ffffffffffffffffffffffffffffffffffffffff8516865261ffff9190911691505f60a085901c60020b60020b602087015260038b01546fffffffffffffffffffffffffffffffff16604087015260808a01515f90624000001661475d5760d086901c62ffffff1661476f565b61476f8b6080015162ffffff1661530c565b9050831561479d57620f4240610fff851662ffffff831681810283810615159390049290920191010361479f565b805b975050620f42408762ffffff16106147e35789515f12156147e3576147e37f9620624600000000000000000000000000000000000000000000000000000000611d1e565b89515f036147fb575f80985098505050505050614f32565b83156148de5760608a015173ffffffffffffffffffffffffffffffffffffffff86811691161061486d5761486d73ffffffffffffffffffffffffffffffffffffffff86165b60608c01517f7c9c6e8f000000000000000000000000000000000000000000000000000000009190612d11565b6401000276a373ffffffffffffffffffffffffffffffffffffffff168a6060015173ffffffffffffffffffffffffffffffffffffffff16116148d95760608a01516148d9907f9e4d7cc70000000000000000000000000000000000000000000000000000000090612e22565b61499c565b60608a015173ffffffffffffffffffffffffffffffffffffffff8681169116116149215761492173ffffffffffffffffffffffffffffffffffffffff8616614840565b73fffd8963efd1fc6a506488495d951d5263988d2673ffffffffffffffffffffffffffffffffffffffff168a6060015173ffffffffffffffffffffffffffffffffffffffff161061499c5760608a015161499c907f9e4d7cc70000000000000000000000000000000000000000000000000000000090612e22565b60408051610100810182525f80825260208201819052918101829052606081018290526080810182905260a0810182905260c0810182905260e0810191909152846149eb578b600201546149f1565b8b600101545b60e08201525b821580614a3657508a6060015173ffffffffffffffffffffffffffffffffffffffff16875f015173ffffffffffffffffffffffffffffffffffffffff16145b614dbe57865173ffffffffffffffffffffffffffffffffffffffff168152602080880151908c0151614a6d9160058f01918861531b565b1515604083015260020b602082018190527ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2761812614acb577ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff2761860208201525b620d89e860020b816020015160020b12614ae957620d89e860208201525b614af68160200151613f25565b73ffffffffffffffffffffffffffffffffffffffff90811660608381018290528951908e0151614b40939192911680821891811160018a161891909102188960400151868c615446565b60c085015260a0840152608083015273ffffffffffffffffffffffffffffffffffffffff1687528a515f1215614ba957614b7d8160a001516151db565b83039250614b988160c00151826080015161423d91906161a0565b614ba29083616ab5565b9150614bda565b614bbc8160c001518260800151016151db565b83019250614bcd8160a001516151db565b614bd79083616887565b91505b8315614c16575f620f4240858360c001518460800151010281614bff57614bff616a50565b60c084018051929091049182900390529990990198505b60408701516fffffffffffffffffffffffffffffffff1615614c7557614c698160c0015170010000000000000000000000000000000089604001516fffffffffffffffffffffffffffffffff1691020490565b60e08201805190910190525b806060015173ffffffffffffffffffffffffffffffffffffffff16875f015173ffffffffffffffffffffffffffffffffffffffff1603614d8b57806040015115614d66575f8086614ccf578d600101548360e00151614cda565b8260e001518e600201545b915091505f614d328f85602001518585600292830b5f908152600490940160205260409093206001810180549092039091559081018054909203909155547001000000000000000000000000000000009004600f0b90565b90508715614d3d575f035b614d4b8a6040015182614286565b6fffffffffffffffffffffffffffffffff1660408b01525050505b84614d75578060200151614d7e565b60018160200151035b60020b60208801526149f7565b8051875173ffffffffffffffffffffffffffffffffffffffff908116911614614db9578651614d7e90614385565b6149f7565b86516020880151614e539190614e1590899060a01b76ffffff0000000000000000000000000000000000000000167fffffffffffffffffff000000ffffffffffffffffffffffffffffffffffffffff919091161790565b7fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff9091161790565b8c55604087015160038d01546fffffffffffffffffffffffffffffffff908116911614614ec257604087015160038d0180547fffffffffffffffffffffffffffffffff00000000000000000000000000000000166fffffffffffffffffffffffffffffffff9092169190911790555b84614ed65760e081015160028d0155614ee1565b60e081015160018d01555b8a515f1385151514614f0e57614f07614ef98361267e565b612925858e5f01510361267e565b9950614f2b565b614f28614f1f848d5f01510361267e565b6129258461267e565b99505b5050505050505b92959194509250565b73ffffffffffffffffffffffffffffffffffffffff83165f90815260046020908152604080832085845290915281208054839290614f7a90849061618d565b9091555050604080513381526020810183905283915f9173ffffffffffffffffffffffffffffffffffffffff8716917f1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac7288599101612041565b6040518381528260020b60048201528160020b6024820152604481fd5b5f838302817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8587098281108382030391505080841161502c575f80fd5b805f0361503e57508290049050610838565b5f848688095f868103871696879004966002600389028118808a02820302808a02820302808a02820302808a02820302808a02820302808a02909103029181900381900460010186841190950394909402919094039290920491909117919091029150509392505050565b5f8373ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff1611156150e2579293925b73ffffffffffffffffffffffffffffffffffffffff85166151095762bfc9215f526004601cfd5b7bffffffffffffffffffffffffffffffff000000000000000000000000606084901b1673ffffffffffffffffffffffffffffffffffffffff8686031683615195578673ffffffffffffffffffffffffffffffffffffffff1661518283838973ffffffffffffffffffffffffffffffffffffffff16614fee565b8161518f5761518f616a50565b0461309d565b61309d6151b983838973ffffffffffffffffffffffffffffffffffffffff166155b6565b8873ffffffffffffffffffffffffffffffffffffffff16808204910615150190565b805f811215610c6257610c627f93dafdf100000000000000000000000000000000000000000000000000000000611d1e565b5f73ffffffffffffffffffffffffffffffffffffffff8481169086160360ff81901d908101186c010000000000000000000000006fffffffffffffffffffffffffffffffff851661525f818484614fee565b9350845f83858409111684019350505050949350505050565b5f808211615284575f80fd5b507f0706060506020500060203020504000106050205030304010505030400000000601f6f8421084210842108cc6318c6db6d54be6fffffffffffffffffffffffffffffffff841160071b84811c67ffffffffffffffff1060061b1784811c63ffffffff1060051b1784811c61ffff1060041b1784811c60ff1060031b1793841c1c161a1790565b62bfffff8116610c6281612425565b5f80600284810b9086900b81810783139190050383156153b957600281900b60081d600181900b5f908152602089905260409020547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60ff808516908190039190911c91821680151595509091908561539b57888360ff168603026153ae565b886153a582615278565b840360ff168603025b96505050505061543c565b6001908101600281900b60081d80830b5f90815260208a905260409020547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60ff841694851b01199081168015159550929391928561542257888360ff0360ff16860102615435565b888361542d836155e6565b0360ff168601025b9650505050505b5094509492505050565b5f80808062ffffff851673ffffffffffffffffffffffffffffffffffffffff808a16908b1610158288128015615529575f61548c8a5f0385620f424003620f4240614fee565b9050826154a5576154a08d8d8d600161520d565b6154b2565b6154b28c8e8d60016150a9565b96508681106154e6578b9750620f424084146154dd576154d8878586620f4240036155b6565b6154df565b865b94506154ff565b8096506154f58d8c8386615680565b9750868a5f030394505b82615515576155108d898d5f6150a9565b615521565b615521888e8d5f61520d565b9550506155a7565b8161553f5761553a8c8c8c5f6150a9565b61554b565b61554b8b8d8c5f61520d565b945084891061555c578a965061556e565b88945061556b8c8b87856156e4565b96505b81615585576155808c888c600161520d565b615592565b615592878d8c60016150a9565b95506155a4868485620f4240036155b6565b93505b50505095509550955095915050565b5f6155c2848484614fee565b905081806155d2576155d2616a50565b838509156108385760010180610838575f80fd5b5f8082116155f2575f80fd5b507e1f0d1e100c1d070f090b19131c1706010e11080a1a141802121b15031604055f8290039091166101e07f804040554300526644320000502061067405302602000010750620017611707760fc7fb6db6db6ddddddddd34d34d349249249210842108c6318c639ce739cffffffff840260f81c161b60f71c1690811c63d76453e004601f169190911a1790565b5f6fffffffffffffffffffffffffffffffff84161573ffffffffffffffffffffffffffffffffffffffff86161517156156c057634f2461b85f526004601cfd5b816156d7576156d2858585600161573d565b612b22565b612b22858585600161589f565b5f6fffffffffffffffffffffffffffffffff84161573ffffffffffffffffffffffffffffffffffffffff861615171561572457634f2461b85f526004601cfd5b81615735576156d28585855f61589f565b612b228585855f5b5f81156157e2575f73ffffffffffffffffffffffffffffffffffffffff8411156157905761578b846c01000000000000000000000000876fffffffffffffffffffffffffffffffff16614fee565b6157b0565b6157b06fffffffffffffffffffffffffffffffff8616606086901b616a7d565b90506157da6157d58273ffffffffffffffffffffffffffffffffffffffff89166161a0565b6159d4565b915050611d16565b5f73ffffffffffffffffffffffffffffffffffffffff84111561582e57615829846c01000000000000000000000000876fffffffffffffffffffffffffffffffff166155b6565b615854565b615854606085901b6fffffffffffffffffffffffffffffffff8716808204910615150190565b90508073ffffffffffffffffffffffffffffffffffffffff87161161588057634323a5555f526004601cfd5b73ffffffffffffffffffffffffffffffffffffffff8616039050611d16565b5f825f036158ae575083611d16565b7bffffffffffffffffffffffffffffffff000000000000000000000000606085901b1682156159795773ffffffffffffffffffffffffffffffffffffffff86168481029085828161590157615901616a50565b040361593e5781810182811061593c57615932838973ffffffffffffffffffffffffffffffffffffffff16836155b6565b9350505050611d16565b505b506157da818561596473ffffffffffffffffffffffffffffffffffffffff8a1683616a7d565b61596e91906161a0565b808204910615150190565b73ffffffffffffffffffffffffffffffffffffffff86168481029085820414818311166159ad5763f5c787f15f526004601cfd5b8082036159326157d58473ffffffffffffffffffffffffffffffffffffffff8b16846155b6565b8073ffffffffffffffffffffffffffffffffffffffff81168114610c6257610c627f93dafdf100000000000000000000000000000000000000000000000000000000611d1e565b73ffffffffffffffffffffffffffffffffffffffff81168114611799575f80fd5b5f8060408385031215615a4d575f80fd5b8235615a5881615a1b565b946020939093013593505050565b5f60208284031215615a76575f80fd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114610838575f80fd5b5f805f60608486031215615ab7575f80fd5b8335615ac281615a1b565b95602085013595506040909401359392505050565b5f805f60608486031215615ae9575f80fd5b8335615af481615a1b565b92506020840135615b0481615a1b565b929592945050506040919091013590565b5f60208284031215615b25575f80fd5b5035919050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52604160045260245ffd5b6040516080810167ffffffffffffffff81118282101715615b7c57615b7c615b2c565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715615bc957615bc9615b2c565b604052919050565b803562ffffff81168114610c62575f80fd5b8035600281900b8114610c62575f80fd5b5f60a08284031215615c04575f80fd5b60405160a0810167ffffffffffffffff81118282101715615c2757615c27615b2c565b6040529050808235615c3881615a1b565b81526020830135615c4881615a1b565b6020820152615c5960408401615bd1565b6040820152615c6a60608401615be3565b60608201526080830135615c7d81615a1b565b6080919091015292915050565b5f8083601f840112615c9a575f80fd5b50813567ffffffffffffffff811115615cb1575f80fd5b602083019150836020828501011115615cc8575f80fd5b9250929050565b5f805f805f6101008688031215615ce4575f80fd5b615cee8787615bf4565b945060a0860135935060c0860135925060e086013567ffffffffffffffff811115615d17575f80fd5b615d2388828901615c8a565b969995985093965092949392505050565b5f60208284031215615d44575f80fd5b813561083881615a1b565b5f8060408385031215615d60575f80fd5b50508035926020909101359150565b602080825282518282018190525f918401906040840190835b81811015615da6578351835260209384019390920191600101615d88565b509095945050505050565b5f8060208385031215615dc2575f80fd5b823567ffffffffffffffff811115615dd8575f80fd5b615de485828601615c8a565b90969095509350505050565b602081525f82518060208401528060208501604085015e5f6040828501015260407fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011684010191505092915050565b5f8060c08385031215615e54575f80fd5b615e5e8484615bf4565b9150615e6c60a08401615bd1565b90509250929050565b80358015158114610c62575f80fd5b5f8060408385031215615e95575f80fd5b8235615ea081615a1b565b9150615e6c60208401615e75565b5f805f80848603610140811215615ec3575f80fd5b615ecd8787615bf4565b945060807fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6082011215615efe575f80fd5b50615f07615b59565b615f1360a08701615be3565b8152615f2160c08701615be3565b602082015260e086013560408201526101008601356060820152925061012085013567ffffffffffffffff811115615f57575f80fd5b615f6387828801615c8a565b95989497509550505050565b5f805f8060e08587031215615f82575f80fd5b615f8c8686615bf4565b935060a0850135615f9c81615a1b565b925060c085013567ffffffffffffffff811115615f57575f80fd5b5f8060208385031215615fc8575f80fd5b823567ffffffffffffffff811115615fde575f80fd5b8301601f81018513615fee575f80fd5b803567ffffffffffffffff811115616004575f80fd5b8560208260051b8401011115616018575f80fd5b6020919091019590945092505050565b5f8060408385031215616039575f80fd5b823561604481615a1b565b9150602083013561605481615a1b565b809150509250929050565b5f805f80848603610120811215616074575f80fd5b61607e8787615bf4565b945060607fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60820112156160af575f80fd5b506040516060810167ffffffffffffffff811182821017156160d3576160d3615b2c565b6040526160e260a08701615e75565b815260c0860135602082015260e08601356160fc81615a1b565b6040820152925061010085013567ffffffffffffffff811115615f57575f80fd5b5f805f8060808587031215616130575f80fd5b843561613b81615a1b565b9350602085013561614b81615a1b565b93969395505050506040820135916060013590565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b8181038181111561076557610765616160565b8082018082111561076557610765616160565b81835281816020850137505f602082840101525f60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f840116840101905092915050565b602081525f611d166020830184866161b3565b5f6020828403121561621d575f80fd5b815167ffffffffffffffff811115616233575f80fd5b8201601f81018413616243575f80fd5b805167ffffffffffffffff81111561625d5761625d615b2c565b61628e60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601615b82565b8181528560208385010111156162a2575f80fd5b8160208401602083015e5f91810160200191909152949350505050565b73ffffffffffffffffffffffffffffffffffffffff8716815261635a602082018773ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8460c08201528360e08201526101206101008201525f61637f610120830184866161b3565b98975050505050505050565b73ffffffffffffffffffffffffffffffffffffffff86168152616426602082018673ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8351600290810b60c08301526020850151900b60e0820152604084015161010082015260608401516101208201526101606101408201525f61309d610160830184866161b3565b73ffffffffffffffffffffffffffffffffffffffff88168152616508602082018873ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8551600290810b60c08301526020870151900b60e08201526040860151610100820152606086015161012082015284610140820152836101608201526101a06101808201525f61469c6101a0830184866161b3565b73ffffffffffffffffffffffffffffffffffffffff861681526165f8602082018673ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff841660c082015261010060e08201525f61309d610100830184866161b3565b60a08101610765828473ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff8716815261674a602082018773ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b73ffffffffffffffffffffffffffffffffffffffff851660c08201528360020b60e08201526101206101008201525f61637f610120830184866161b3565b5f60208284031215616798575f80fd5b5051919050565b73ffffffffffffffffffffffffffffffffffffffff8616815261683a602082018673ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8351151560c0820152602084015160e0820152604084015173ffffffffffffffffffffffffffffffffffffffff166101008201526101406101208201525f61309d610140830184866161b3565b8082018281125f8312801582168215821617156168a6576168a6616160565b505092915050565b73ffffffffffffffffffffffffffffffffffffffff87168152616949602082018773ffffffffffffffffffffffffffffffffffffffff815116825273ffffffffffffffffffffffffffffffffffffffff602082015116602083015262ffffff6040820151166040830152606081015160020b606083015273ffffffffffffffffffffffffffffffffffffffff60808201511660808301525050565b8451151560c0820152602085015160e0820152604085015173ffffffffffffffffffffffffffffffffffffffff16610100820152836101208201526101606101408201525f61637f610160830184866161b3565b600f81810b9083900b016f7fffffffffffffffffffffffffffffff81137fffffffffffffffffffffffffffffffff800000000000000000000000000000008212171561076557610765616160565b600f82810b9082900b037fffffffffffffffffffffffffffffffff8000000000000000000000000000000081126f7fffffffffffffffffffffffffffffff8213171561076557610765616160565b808202811582820484141761076557610765616160565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f82616ab0577f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b500490565b8181035f831280158383131683831282161715613b3757613b3761616056fea164736f6c634300081a000a
    /// ```
    #[rustfmt::skip]
    #[allow(clippy::all)]
    pub static DEPLOYED_BYTECODE: alloy_sol_types::private::Bytes = alloy_sol_types::private::Bytes::from_static(
        b"`\x80`@R`\x046\x10a\x01\xF4W_5`\xE0\x1C\x80cZk\xCF\xDA\x11a\x01\x17W\x80c\xA5\x84\x11\x94\x11a\0\xACW\x80c\xF15\xBA\xAA\x11a\0|W\x80c\xF3\xCD\x91L\x11a\0bW\x80c\xF3\xCD\x91L\x14a\x06vW\x80c\xF5)\x8A\xCA\x14a\x06\x95W\x80c\xFE\x99\x04\x9A\x14a\x06\xB4W_\x80\xFD[\x80c\xF15\xBA\xAA\x14a\x068W\x80c\xF2\xFD\xE3\x8B\x14a\x06WW_\x80\xFD[\x80c\xA5\x84\x11\x94\x14a\x05\x95W\x80c\xB66<\xF2\x14a\x05\xB4W\x80c\xDB\xD05\xFF\x14a\x05\xEDW\x80c\xF0-\xE3\xB2\x14a\x06\x0CW_\x80\xFD[\x80c\x81a\xB8t\x11a\0\xE7W\x80c\x81a\xB8t\x14a\x04\xDCW\x80c\x8D\xA5\xCB[\x14a\x04\xFBW\x80c\x97\xE8\xCDN\x14a\x05KW\x80c\x9B\xF6d_\x14a\x05vW_\x80\xFD[\x80cZk\xCF\xDA\x14a\x048W\x80ci\\[\xF5\x14a\x04lW\x80c~\x87\xCE}\x14a\x04\x9EW\x80c\x80\xF0\xB4L\x14a\x04\xBDW_\x80\xFD[\x80c-w\x13\x89\x11a\x01\x8DW\x80cH\xC8\x94\x91\x11a\x01]W\x80cH\xC8\x94\x91\x14a\x03\x92W\x80cRu\x96Q\x14a\x03\xBEW\x80cU\x8Ar\x97\x14a\x03\xDDW\x80cY\x8A\xF9\xE7\x14a\x03\xFCW_\x80\xFD[\x80c-w\x13\x89\x14a\x03\x15W\x80c5\xFDc\x1A\x14a\x034W\x80c=\xD4Z\xDB\x14a\x03`W\x80cBj\x84\x93\x14a\x03sW_\x80\xFD[\x80c\x11\xDA`\xB4\x11a\x01\xC8W\x80c\x11\xDA`\xB4\x14a\x02\xB0W\x80c\x15n)\xF6\x14a\x02\xB8W\x80c\x1E.\xAE\xAF\x14a\x02\xD7W\x80c#Bf\xD7\x14a\x02\xF6W_\x80\xFD[\x80b\xFD\xD5\x8E\x14a\x01\xF8W\x80c\x01\xFF\xC9\xA7\x14a\x02AW\x80c\t[\xCD\xB6\x14a\x02pW\x80c\x0B\r\x9C\t\x14a\x02\x8FW[_\x80\xFD[4\x80\x15a\x02\x03W_\x80\xFD[Pa\x02.a\x02\x126`\x04aZ<V[`\x04` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T\x81V[`@Q\x90\x81R` \x01[`@Q\x80\x91\x03\x90\xF3[4\x80\x15a\x02LW_\x80\xFD[Pa\x02`a\x02[6`\x04aZfV[a\x06\xD3V[`@Q\x90\x15\x15\x81R` \x01a\x028V[4\x80\x15a\x02{W_\x80\xFD[Pa\x02`a\x02\x8A6`\x04aZ\xA5V[a\x07kV[4\x80\x15a\x02\x9AW_\x80\xFD[Pa\x02\xAEa\x02\xA96`\x04aZ\xD7V[a\x08?V[\0[a\x02.a\x08\xC9V[4\x80\x15a\x02\xC3W_\x80\xFD[Pa\x02\xAEa\x02\xD26`\x04aZ\xA5V[a\t'V[4\x80\x15a\x02\xE2W_\x80\xFD[Pa\x02.a\x02\xF16`\x04a[\x15V[a\t\xABV[4\x80\x15a\x03\x01W_\x80\xFD[Pa\x02.a\x03\x106`\x04a\\\xCFV[a\t\xB5V[4\x80\x15a\x03 W_\x80\xFD[Pa\x02\xAEa\x03/6`\x04a]4V[a\n\xD9V[4\x80\x15a\x03?W_\x80\xFD[Pa\x03Sa\x03N6`\x04a]OV[a\x0B\xCCV[`@Qa\x028\x91\x90a]oV[a\x02.a\x03n6`\x04a]4V[a\x0C\tV[4\x80\x15a\x03~W_\x80\xFD[Pa\x02`a\x03\x8D6`\x04aZ\xA5V[a\x0CgV[4\x80\x15a\x03\x9DW_\x80\xFD[Pa\x03\xB1a\x03\xAC6`\x04a]\xB1V[a\x0C\xD8V[`@Qa\x028\x91\x90a]\xF0V[4\x80\x15a\x03\xC9W_\x80\xFD[Pa\x02\xAEa\x03\xD86`\x04a^CV[a\x0E*V[4\x80\x15a\x03\xE8W_\x80\xFD[Pa\x02`a\x03\xF76`\x04a^\x84V[a\x0E\xCCV[4\x80\x15a\x04\x07W_\x80\xFD[Pa\x02.a\x04\x166`\x04aZ\xD7V[`\x05` \x90\x81R_\x93\x84R`@\x80\x85 \x82R\x92\x84R\x82\x84 \x90R\x82R\x90 T\x81V[4\x80\x15a\x04CW_\x80\xFD[Pa\x04Wa\x04R6`\x04a^\xAEV[a\x0FfV[`@\x80Q\x92\x83R` \x83\x01\x91\x90\x91R\x01a\x028V[4\x80\x15a\x04wW_\x80\xFD[Pa\x04\x8Ba\x04\x866`\x04a_oV[a\x11eV[`@Q`\x02\x91\x90\x91\x0B\x81R` \x01a\x028V[4\x80\x15a\x04\xA9W_\x80\xFD[Pa\x02\xAEa\x04\xB86`\x04a^CV[a\x14\x12V[4\x80\x15a\x04\xC8W_\x80\xFD[Pa\x02\xAEa\x04\xD76`\x04aZ<V[a\x15\x03V[4\x80\x15a\x04\xE7W_\x80\xFD[Pa\x02.a\x04\xF66`\x04aZ\xD7V[a\x15\xC2V[4\x80\x15a\x05\x06W_\x80\xFD[P_Ta\x05&\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x81R` \x01a\x028V[4\x80\x15a\x05VW_\x80\xFD[Pa\x02.a\x05e6`\x04a]4V[`\x01` R_\x90\x81R`@\x90 T\x81V[4\x80\x15a\x05\x81W_\x80\xFD[Pa\x03Sa\x05\x906`\x04a_\xB7V[a\x16\xEEV[4\x80\x15a\x05\xA0W_\x80\xFD[Pa\x02\xAEa\x05\xAF6`\x04a]4V[a\x17'V[4\x80\x15a\x05\xBFW_\x80\xFD[Pa\x02`a\x05\xCE6`\x04a`(V[`\x03` \x90\x81R_\x92\x83R`@\x80\x84 \x90\x91R\x90\x82R\x90 T`\xFF\x16\x81V[4\x80\x15a\x05\xF8W_\x80\xFD[Pa\x03Sa\x06\x076`\x04a_\xB7V[a\x17\xCCV[4\x80\x15a\x06\x17W_\x80\xFD[P`\x02Ta\x05&\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81V[4\x80\x15a\x06CW_\x80\xFD[Pa\x02.a\x06R6`\x04a[\x15V[a\x18\x03V[4\x80\x15a\x06bW_\x80\xFD[Pa\x02\xAEa\x06q6`\x04a]4V[a\x18\rV[4\x80\x15a\x06\x81W_\x80\xFD[Pa\x02.a\x06\x906`\x04a`_V[a\x18\xFCV[4\x80\x15a\x06\xA0W_\x80\xFD[Pa\x02\xAEa\x06\xAF6`\x04aZ\xA5V[a\x1A\xAEV[4\x80\x15a\x06\xBFW_\x80\xFD[Pa\x02`a\x06\xCE6`\x04aa\x1DV[a\x1B2V[_\x7F\x01\xFF\xC9\xA7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14\x80a\x07eWP\x7F\x0Fc/\xB3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x16\x14[\x92\x91PPV[3_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x91\x90\x83\x90a\x07\x96\x90\x84\x90aa\x8DV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x07\xDA\x90\x84\x90aa\xA0V[\x90\x91UPP`@\x80Q3\x80\x82R` \x82\x01\x85\x90R\x85\x92s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x92\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01[`@Q\x80\x91\x03\x90\xA4P`\x01[\x93\x92PPPV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x08\x8EWa\x08\x8E\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x08\xA3\x83a\x08\x9B\x83a\x1D&V[_\x033a\x1DkV[a\x08\xC4s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x83\x83a\x1D\xCBV[PPPV[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\t\x19Wa\t\x19\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\t\"3a\x1E\xC6V[\x90P\x90V[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\tvWa\tv\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x81a\t\x84\x81a\x08\x9B\x84a\x1D&V[a\t\xA5\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x84a\x1F\xAFV[PPPPV[_\x81T_R` _\xF3[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\n\x05Wa\n\x05\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\n\ra NV[`\xA0\x86 _\x81\x81R`\x06` R`@\x90 a\n'\x81a \xB6V[`\x80\x88\x01Qa\nP\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x89\x89\x89a \xFDV[a\n[\x81\x88\x88a!\xE0V[\x92Pa\nh\x88\x843a\"\xDAV[`@\x80Q\x88\x81R` \x81\x01\x88\x90R3\x91\x84\x91\x7F)\xEF\x05\xCA\xAF\xF9@K|\xB6\xD1\xC0\xE9\xBB\xAE\x9E\xAAz\xB2T\x1F\xEB\xA1\xA9\xC4$\x85\x94\xC0\x81V\xCB\x91\x01`@Q\x80\x91\x03\x90\xA3`\x80\x88\x01Qa\n\xCE\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x89\x89\x89a#\x02V[PP\x95\x94PPPPPV[_Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x0B^W`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01[`@Q\x80\x91\x03\x90\xFD[`\x02\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x90\x91U`@Q\x7F\xB4\xBD\x8E\xF5=\xF6\x90\xB9\x94=3\x18\x99`\x06\xDB\xB8*%\xF5G\x19\xD8\xC8\x03[Qj*[\x8A\xCC\x90_\x90\xA2PV[```@Q\x80\x83`\x05\x1B` \x83R\x84` \x84\x01R`@\x83\x01\x92P\x80\x83\x01\x90P[\x85T\x83R` \x83\x01\x92P`\x01\x86\x01\x95P\x80\x83\x10a\x0B\xECW\x81\x81\x03\x82\xF3[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x0CYWa\x0CY\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x07e\x82a\x1E\xC6V[\x91\x90PV[3_\x81\x81R`\x05` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x80\x85R\x90\x83R\x81\x84 \x87\x85R\x90\x92R\x80\x83 \x85\x90UQ\x91\x92\x85\x92\x7F\xB3\xFDPq\x83X\x87Vz\x06q\x15\x11!\x89M\xDC\xCC(B\xF1\xD1\x0B\xED\xAD\x13\xE0\xD1|\xAC\xE9\xA7\x90a\x08,\x90\x87\x81R` \x01\x90V[``\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\\x15a\r*Wa\r*\x7FP\x90\xD6\xC6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\r2a#\xDAV[`@Q\x7F\x91\xDDsF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R3\x90c\x91\xDDsF\x90a\rp\x90\x86\x90\x86\x90`\x04\x01aa\xFAV[_`@Q\x80\x83\x03\x81_\x87Z\xF1\x15\x80\x15a\r\x8BW=_\x80>=_\xFD[PPPP`@Q=_\x82>`\x1F=\x90\x81\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x82\x01`@Ra\r\xD0\x91\x90\x81\x01\x90ab\rV[\x90P\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B\\\x15a\x0E\"Wa\x0E\"\x7FR\x12\xCB\xA1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x07ea$\0V[`@\x82\x01Qb\xFF\xFF\xFF\x16b\x80\0\0\x14\x15\x80a\x0EuWP\x81`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14\x15[\x15a\x0E\xA3Wa\x0E\xA3\x7F0\xD2\x16A\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x0E\xB1\x81b\xFF\xFF\xFF\x16a$%V[`\xA0\x82 _\x81\x81R`\x06` R`@\x90 a\x08\xC4\x90\x83a$dV[3_\x81\x81R`\x03` \x90\x81R`@\x80\x83 s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x80\x85R\x90\x83R\x81\x84 \x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\x16\x87\x15\x15\x90\x81\x17\x90\x91U\x91Q\x91\x82R\x92\x93\x91\x7F\xCE\xB5v\xD9\xF1^N \x0F\xDBP\x96\xD6M]\xFDf~\x16\xDE\xF2\x0C\x1E\xEF\xD1BV\xD8\xE3\xFA\xA2g\x91\x01`@Q\x80\x91\x03\x90\xA3P`\x01\x92\x91PPV[_\x80\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x0F\xB7Wa\x0F\xB7\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x0F\xBFa NV[`\xA0\x86 _\x81\x81R`\x06` R`@\x90 a\x0F\xD9\x81a \xB6V[`\x80\x88\x01Qa\x10\x01\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x89\x89a$\xBDV[_a\x10u`@Q\x80`\xC0\x01`@R\x803s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x8A_\x01Q`\x02\x0B\x81R` \x01\x8A` \x01Q`\x02\x0B\x81R` \x01a\x10R\x8B`@\x01Qa&~V[`\x0F\x0B\x81R``\x80\x8D\x01Q`\x02\x0B` \x83\x01R\x8B\x01Q`@\x90\x91\x01R\x83\x90a&\xB3V[\x94P\x90Pa\x10\x83\x81\x85a*\xF8V[\x94PPP3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81\x7F\xF2\x08\xF4\x91'\x82\xFD%\xC7\xF1\x14\xCA7#\xA2\xD5\xDDo;\xCC:\xC8\xDBZ\xF6;\xAA\x85\xF7\x11\xD5\xEC\x88_\x01Q\x89` \x01Q\x8A`@\x01Q\x8B``\x01Q`@Qa\x11\x01\x94\x93\x92\x91\x90`\x02\x94\x85\x0B\x81R\x92\x90\x93\x0B` \x83\x01R`@\x82\x01R``\x81\x01\x91\x90\x91R`\x80\x01\x90V[`@Q\x80\x91\x03\x90\xA3`\x80\x87\x01Q_\x90a\x115\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x87\x87\x8B\x8Ba++V[\x90\x94P\x90P\x80\x15a\x11OWa\x11O\x88\x82\x8A`\x80\x01Qa\"\xDAV[a\x11Z\x88\x853a\"\xDAV[PP\x94P\x94\x92PPPV[_a\x11na NV[``\x85\x01Qa\x7F\xFF`\x02\x91\x90\x91\x0B\x13\x15a\x11\xB2W``\x85\x01Qa\x11\xB2\x90\x7F\xB7\0$\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a-\x02V[`\x01`\x02\x0B\x85``\x01Q`\x02\x0B\x12\x15a\x11\xF5W``\x85\x01Qa\x11\xF5\x90\x7F\xE9\xE9\x05\x88\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a-\x02V[\x84Q` \x86\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x10a\x12MW\x84Q` \x86\x01Qa\x12M\x91\x7Fnl\x980\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91a-\x11V[a\x12~\x85`@\x01Q\x86`\x80\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a-T\x90\x91\x90c\xFF\xFF\xFF\xFF\x16V[a\x12\xB2W`\x80\x85\x01Qa\x12\xB2\x90\x7F\xE6Z\xF6\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a.\"V[_a\x12\xC5\x86`@\x01Qb\xFF\xFF\xFF\x16a.DV[`\x80\x87\x01Q\x90\x91Pa\x12\xF0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87\x87\x87\x87a.iV[`\xA0\x86 _a\x12\xFE\x88a/@V[_\x83\x81R`\x06` R`@\x90 \x90\x91Pa\x13\x1A\x90\x88\x83\x86a0\xA8V[`\x80\x89\x01Q\x90\x94Pa\x13F\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x89\x89\x87\x8A\x8Aa1\x83V[\x87` \x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x88_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83\x7F\xDDFngN\xA5W\xF5b\x95\xE2\xD0!\x8A\x12^\xA4\xB4\xF0\xF6\xF30{\x95\xF8^a\x10\x83\x8Dd8\x8B`@\x01Q\x8C``\x01Q\x8D`\x80\x01Q\x8D\x8B`@Qa\x13\xFF\x95\x94\x93\x92\x91\x90b\xFF\xFF\xFF\x95\x90\x95\x16\x85R`\x02\x93\x84\x0B` \x86\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x83\x16`@\x86\x01R\x91\x16``\x84\x01R\x90\x0B`\x80\x82\x01R`\xA0\x01\x90V[`@Q\x80\x91\x03\x90\xA4PPP\x94\x93PPPPV[`\x02Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x14ZWa\x14Z\x7FH\xF5\xC3\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x03\xE9a\x0F\xFF\x82\x16\x10b>\x90\0b\xFF\xF0\0\x83\x16\x10\x16a\x14\xA2Wa\x14\xA2\x7F\xA7\xAB\xE2\xF7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x83\x16a.\"V[`\xA0\x82 a\x14\xC4\x82a\x14\xBE\x83_\x90\x81R`\x06` R`@\x90 \x90V[\x90a2\\V[`@Qb\xFF\xFF\xFF\x83\x16\x81R\x81\x90\x7F\xE9\xC4%\x93\xE7\x1F\x84@;\x845,\xD1h\xD6\x93\xE2\xC9\xFC\xD1\xFD\xBC\xC3\xFE\xB2\x1D\x92\xB4>f\x96\xF9\x90` \x01`@Q\x80\x91\x03\x90\xA2PPPV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x15RWa\x15R\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[3_\x90\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16` R`@\x81 \\\x90a\x15\x80\x83a\x1D&V[\x90P\x81\x81`\x0F\x0B\x14a\x15\xB5Wa\x15\xB5\x7F\xBD\xA7:\xBF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\t\xA5\x84\x82_\x033a\x1DkV[`\x02T_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x16\x0CWa\x16\x0C\x7FH\xF5\xC3\xED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\\x15a\x16\\Wa\x16\\\x7F>_O\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x81\x15a\x16hW\x81a\x16\x8EV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x01` R`@\x90 T[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x90\x81R`\x01` R`@\x81 \x80T\x92\x93P\x83\x92\x90\x91\x90a\x16\xC7\x90\x84\x90aa\x8DV[\x90\x91UPa\x088\x90Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x85\x83a\x1D\xCBV[```@Q\x80` \x82R\x83` \x83\x01R`@\x82\x01\x91P\x83`\x05\x1B\x82\x01\x85[\x805\\\x84R` \x93\x84\x01\x93\x01\x81\x84\x10a\x17\x0CW[P\x81\x81\x03\x82\xF3[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x17vWa\x17v\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x17\x9CWa\x17\x99a2\xB0V[PV[_a\x17\xBC\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a2\xD5V[\x90Pa\x17\xC8\x82\x82a3\x84V[PPV[```@Q\x80` \x82R\x83` \x83\x01R`@\x82\x01\x91P\x83`\x05\x1B\x82\x01\x85[\x805T\x84R` \x93\x84\x01\x93\x01\x81\x84\x10\x15a\x17 Wa\x17\xEAV[_\x81\\_R` _\xF3[_Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x163\x14a\x18\x8DW`@Q\x7F\x08\xC3y\xA0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R` `\x04\x82\x01R`\x0C`$\x82\x01R\x7FUNAUTHORIZED\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`D\x82\x01R`d\x01a\x0BUV[_\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90\x81\x17\x82U`@Q\x90\x913\x91\x7F\x8B\xE0\x07\x9CS\x16Y\x14\x13D\xCD\x1F\xD0\xA4\xF2\x84\x19I\x7F\x97\"\xA3\xDA\xAF\xE3\xB4\x18okdW\xE0\x91\x90\xA3PV[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x19LWa\x19L\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\x19Ta NV[\x83` \x01Q_\x03a\x19\x88Wa\x19\x88\x7F\xBE\x8B\x85\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[`\xA0\x85 _\x81\x81R`\x06` R`@\x90 a\x19\xA2\x81a \xB6V[`\x80\x87\x01Q_\x90\x81\x90\x81\x90a\x19\xD0\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8B\x8B\x8B\x8Ba3\xE4V[\x80\x93P\x81\x95P\x82\x94PPPPa\x1AL\x84\x86`@Q\x80`\xA0\x01`@R\x80\x86\x81R` \x01\x8E``\x01Q`\x02\x0B\x81R` \x01\x8D_\x01Q\x15\x15\x81R` \x01\x8D`@\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x01\x85b\xFF\xFF\xFF\x16\x81RP\x8C_\x01Qa\x1AEW\x8D` \x01Qa5\x8BV[\x8DQa5\x8BV[`\x80\x8B\x01Q\x90\x96P_\x92Pa\x1A}\x91Ps\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8A\x8A\x88\x8B\x8B\x88a6\x8CV[\x90\x95P\x90P\x80\x15a\x1A\x97Wa\x1A\x97\x89\x82\x8B`\x80\x01Qa\"\xDAV[a\x1A\xA2\x89\x863a\"\xDAV[PPPP\x94\x93PPPPV[\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#\\a\x1A\xFDWa\x1A\xFD\x7FT\xE3\xCA\r\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x81a\x1B\x11\x81a\x1B\x0B\x84a\x1D&V[3a\x1DkV[a\t\xA5\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x84a8%V[_3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x14\x80\x15\x90a\x1B\x89WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 3\x84R\x90\x91R\x90 T`\xFF\x16\x15[\x15a\x1C2Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x86\x84R\x90\x91R\x90 T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a\x1C0Wa\x1B\xF7\x83\x82aa\x8DV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 3\x84R\x82R\x80\x83 \x88\x84R\x90\x91R\x90 U[P[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x1Cq\x90\x84\x90aa\x8DV[\x90\x91UPPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x86\x84R\x90\x91R\x81 \x80T\x84\x92\x90a\x1C\xB5\x90\x84\x90aa\xA0V[\x90\x91UPP`@\x80Q3\x81R` \x81\x01\x84\x90R\x84\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x88\x16\x92\x90\x89\x16\x91\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01`@Q\x80\x91\x03\x90\xA4P`\x01[\x94\x93PPPPV[\x80_R`\x04_\xFD[_o\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x10a\x1DgWa\x1Dg\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[P\x90V[\x81`\x0F\x0B_\x03a\x1DzWPPPV[_\x80a\x1D\x9Ds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84\x86a96V[\x91P\x91P\x80_\x03a\x1D\xB5Wa\x1D\xB0a9|V[a\x1D\xC4V[\x81_\x03a\x1D\xC4Wa\x1D\xC4a9\xCAV[PPPPPV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a\x1E%W_\x80_\x80\x85\x87Z\xF1\x90P\x80a\x1E Wa\x1E \x7F\x85I\xDBY\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a:\x18V[a\t\xA5V[`@Q\x7F\xA9\x05\x9C\xBB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\x04\x82\x01R\x82`$\x82\x01R` _`D\x83_\x89Z\xF1=\x15`\x1F=\x11`\x01_Q\x14\x16\x17\x16\x91P_\x81R_` \x82\x01R_`@\x82\x01RP\x80a\t\xA5Wa\t\xA5\x7F\xB1,_\x9C\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a:\x18V[_\x7F'\xE0\x98\xC5\x05\xD4N\xC3W@\x04\xBC\xA0R\xAA\xBFv\xBD5\0L\x18 \x99\xD8\xC5u\xFB#\x85\x93\xB9\\s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16a\x1F\x0CW4\x91Pa\x1F\x96V[4\x15a\x1F;Wa\x1F;\x7F\xB0\xEC\x84\x9E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x7F\x1E\x07E\xA7\xDB\x16#\x98\x1F\x0B*]B26L\0xrf\xEBu\xADTo\x19\x0El\xEB\xE9\xBD\x95\\_a\x1F}s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16a2\xD5V[\x90Pa\x1F\x89\x82\x82aa\x8DV[\x93Pa\x1F\x93a2\xB0V[PP[a\x1F\xA9\x81a\x1F\xA3\x84a\x1D&V[\x85a\x1DkV[P\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x92\x90a\x1F\xEE\x90\x84\x90aa\xA0V[\x90\x91UPP`@\x80Q3\x81R` \x81\x01\x83\x90R\x83\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x91_\x91\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01[`@Q\x80\x91\x03\x90\xA4PPPV[0s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x7F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14a \xB4Wa \xB4\x7F\r\x89C\x8E\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[V[\x80Ts\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a\x17\x99Wa\x17\x99\x7FHj\xA3\x07\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x853s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a!\xD7W` \x87\x16\x15a!\xD7Wa!\xD53\x87\x87\x87\x87\x87`@Q`$\x01a!B\x96\x95\x94\x93\x92\x91\x90ab\xBFV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB6\xA8\xB0\xFA\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a:KV[P[PPPPPPPV[`\x03\x83\x01T_\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x82\x03a\")Wa\")\x7F\xA7O\x97\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a\"`a\"5\x85a\x1D&V[_\x03a\"@\x85a\x1D&V[_\x03`\x80\x91\x90\x91\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x90V[\x91P\x83\x15a\"\x9AW`\x01\x85\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x87\x02\x04\x01\x90U[\x82\x15a\"\xD2W`\x02\x85\x01\x80To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x86\x02\x04\x01\x90U[P\x93\x92PPPV[\x82Qa\"\xF0\x90a\"\xEA\x84`\x80\x1D\x90V[\x83a\x1DkV[a\x08\xC4\x83` \x01Qa\"\xEA\x84`\x0F\x0B\x90V[\x853s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a!\xD7W`\x10\x87\x16\x15a!\xD7Wa!\xD53\x87\x87\x87\x87\x87`@Q`$\x01a#G\x96\x95\x94\x93\x92\x91\x90ab\xBFV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xE1\xB4\xAFi\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a:KV[`\x01\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#]V[_\x7F\xC0\x90\xFCF\x83bL\xFC8\x84\xE9\xD8\xDE^\xCA\x13/-\x0E\xC0b\xAF\xF7]C\xC0F]\\\xEE\xAB#]V[b\x0FB@b\xFF\xFF\xFF\x82\x16\x11\x15a\x17\x99Wa\x17\x99\x7F\x14\0!\x13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0b\xFF\xFF\xFF\x83\x16a.\"V[a$m\x82a \xB6V[\x81T\x7F\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16|\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xD0\x83\x90\x1B\x16\x17[\x90\x91UPV[\x843s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a&vW_\x84`@\x01Q\x13\x80\x15a$\xF1WPa\x08\0\x86\x16\x15\x15[\x15a%\xABWa%\xA53\x86\x86\x86\x86`@Q`$\x01a%\x12\x95\x94\x93\x92\x91\x90ac\x8BV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F%\x99\x82\xE5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90a:KV[Pa&vV[_\x84`@\x01Q\x13\x15\x80\x15a%\xC2WPa\x02\0\x86\x16\x15\x15[\x15a&vWa!\xD73\x86\x86\x86\x86`@Q`$\x01a%\xE3\x95\x94\x93\x92\x91\x90ac\x8BV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F!\xD0\xEEp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90a:KV[PPPPPPV[\x80`\x0F\x81\x90\x0B\x81\x14a\x0CbWa\x0Cb\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[``\x81\x01Q` \x82\x01Q`@\x83\x01Q_\x92\x83\x92\x90\x91a&\xD2\x82\x82a;>V[`@\x80Q`\x80\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x91\x90\x91R\x83`\x0F\x0B_\x14a(\xA2Wa'\x0B\x88\x84\x86_a<\x05V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16` \x83\x01R\x15\x15\x81Ra'3\x88\x83\x86`\x01a<\x05V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x83\x01R\x15\x15`@\x82\x01R_`\x0F\x85\x90\x0B\x12a(gW`\x80\x87\x01Q_\x90`\x02\x0B\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18\x81\x81\x07\x83\x13\x90\x82\x90\x05\x03b\r\x89\xE8\x91\x90\x91\x05\x03`\x01\x01o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04\x90P\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82` \x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a(\x0BWa(\x0B\x7F\xB8\xE3\xC3\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a-\x02V[\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x82``\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15a(eWa(e\x7F\xB8\xE3\xC3\x85\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a-\x02V[P[\x80Q\x15a(\x83W`\x80\x87\x01Qa(\x83\x90`\x05\x8A\x01\x90\x85\x90a<\xEEV[\x80`@\x01Q\x15a(\xA2W`\x80\x87\x01Qa(\xA2\x90`\x05\x8A\x01\x90\x84\x90a<\xEEV[_\x80a(\xAF\x8A\x86\x86a=@V[\x8AQ`\xA0\x8C\x01Q`@\x80Q`&\x81\x01\x92\x90\x92R`\x06\x80\x83\x01\x8A\x90R`\x03\x83\x01\x8B\x90R\x92\x82R`:`\x0C\x83\x01 _\x83\x83\x01\x81\x90R` \x80\x85\x01\x82\x90R\x93\x81\x90R\x90\x81R\x92\x8F\x01\x90\x91R\x81 \x92\x94P\x90\x92P\x80a)\x0C\x83\x8A\x87\x87a=\xF4V[\x91P\x91Pa)Aa)\x1C\x83a\x1D&V[a)%\x83a\x1D&V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x80\x91\x90\x91\x1B\x17\x90V[\x99PPPPPP_\x84`\x0F\x0B\x12\x15a)\xAAW\x80Q\x15a)}W`\x02\x83\x81\x0B_\x90\x81R`\x04\x8A\x01` R`@\x81 \x81\x81U`\x01\x81\x01\x82\x90U\x90\x91\x01U[\x80`@\x01Q\x15a)\xAAW`\x02\x82\x81\x0B_\x90\x81R`\x04\x8A\x01` R`@\x81 \x81\x81U`\x01\x81\x01\x82\x90U\x90\x91\x01U[P\x82`\x0F\x0B_\x14a*\xEEW\x86T_\x80a)\xC6\x83`\xA0\x1C`\x02\x0B\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x91P\x91P\x84`\x02\x0B\x82`\x02\x0B\x12\x15a*\"Wa*\x1Ba*\x15a*\x10a*\x01\x88a?%V[a*\n\x88a?%V[\x8AaB\"V[a&~V[`\x80\x1B\x90V[\x97Pa*\xEAV[\x83`\x02\x0B\x82`\x02\x0B\x12\x15a*\xC5Wa*Ya*Ca*\x10\x83a*\n\x88a?%V[a)%a*\x10a*R\x89a?%V[\x85\x8BaBZV[`\x03\x8B\x01T\x90\x98Pa*}\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87aB\x86V[`\x03\x8B\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x92\x90\x92\x16\x91\x90\x91\x17\x90Ua*\xEAV[a*\xE7_a)%a*\x10a*\xD8\x89a?%V[a*\xE1\x89a?%V[\x8BaBZV[\x97P[PPP[PPP\x92P\x92\x90PV[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x01`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x01a+\"a+\x19\x83a&~V[a)%\x83a&~V[\x95\x94PPPPPV[_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x163\x03a+TWP\x84\x90P_a,\xF6V[\x85\x91P_\x87`@\x01Q\x13\x15a,<Wa\x04\0\x89\x16\x15a,7Wa,(3\x89\x89\x89\x89\x89\x89`@Q`$\x01a+\x8D\x97\x96\x95\x94\x93\x92\x91\x90admV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\x9F\x06>\xFC\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x02\x8B\x16\x15\x15[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8C\x16\x91\x90aB\xB6V[\x90Pa,4\x82\x82aC\x10V[\x91P[a,\xF6V[a\x01\0\x89\x16\x15a,\xF6Wa,\xE73\x89\x89\x89\x89\x89\x89`@Q`$\x01a,f\x97\x96\x95\x94\x93\x92\x91\x90admV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7Fl+\xBE~\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x01\x8B\x16\x15\x15a,\nV[\x90Pa,\xF3\x82\x82aC\x10V[\x91P[\x97P\x97\x95PPPPPPV[\x81_R\x80`\x02\x0B`\x04R`$_\xFD[`@Q\x83\x81Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`$\x82\x01R`D\x81\xFD[_`\x80\x83\x16\x15\x80\x15a-hWP`\x08\x83\x16\x15\x15[\x15a-tWP_a\x07eV[`@\x83\x16\x15\x80\x15a-\x87WP`\x04\x83\x16\x15\x15[\x15a-\x93WP_a\x07eV[a\x04\0\x83\x16\x15\x80\x15a-\xA7WP`\x02\x83\x16\x15\x15[\x15a-\xB3WP_a\x07eV[a\x01\0\x83\x16\x15\x80\x15a-\xC7WP`\x01\x83\x16\x15\x15[\x15a-\xD3WP_a\x07eV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x15a.\x11Wa?\xFF\x83\x16\x15\x15\x80a.\x0CWPb\x80\0\0b\xFF\xFF\xFF\x83\x16\x14a\x088V[a\x088V[Pb\xFF\xFF\xFF\x16b\x80\0\0\x14\x15\x91\x90PV[\x81_Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16`\x04R`$_\xFD[_b\x80\0\0b\xFF\xFF\xFF\x83\x16\x03a.[WP_\x91\x90PV[a\x1Dg\x82b\xFF\xFF\xFF\x16a$%V[\x843s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a&vWa \0\x86\x16\x15a&vWa!\xD73\x86\x86\x86\x86`@Q`$\x01a.\xAD\x95\x94\x93\x92\x91\x90ae]V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F4@\xD8 \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x90a:KV[`\x02T_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a\x0CbW_a/lE`daC1V[\x90P\x80Z\x10\x15a/\x9FWa/\x9F\x7F\x1E\xE4\x97\x02\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[`\x02T`@Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x90_\x90a/\xCE\x90\x86\x90`$\x01af,V[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x80\x82\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FU;\xFC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x81R\x82Q\x92\x93P_\x92\x83\x92\x91\x83\x91\x90\x82\x88\x8A\xF1_Q=` \x14\x90\x91\x16\x92P\x90P\x81\x80\x15a0tWP\x80b\xFF\xFF\xFF\x16\x81\x14[\x80\x15a0\x91WPa\x03\xE9a\x0F\xFF\x82\x16\x10b>\x90\0b\xFF\xF0\0\x83\x16\x10\x16[a0\x9BW_a0\x9DV[\x80[\x97\x96PPPPPPPV[\x83T_\x90s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15a0\xF0Wa0\xF0\x7Fy\x83\xC0Q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a0\xF9\x84aC\x85V[\x90P|\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xD0\x83\x90\x1B\x16v\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xA0\x83\x90\x1B\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x17`\xB8\x85\x90\x1By\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x17\x17\x90\x94UP\x91\x92\x91PPV[\x853s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x14a!\xD7Wa\x10\0\x87\x16\x15a!\xD7Wa!\xD53\x87\x87\x87\x87\x87`@Q`$\x01a1\xC9\x96\x95\x94\x93\x92\x91\x90af\xAFV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xA9\x10\xF8\x0F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16\x90a:KV[a2e\x82a \xB6V[\x81T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16y\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0`\xB8\x83\x90\x1B\x16\x17a$\xB7V[_\x7F'\xE0\x98\xC5\x05\xD4N\xC3W@\x04\xBC\xA0R\xAA\xBFv\xBD5\0L\x18 \x99\xD8\xC5u\xFB#\x85\x93\xB9]V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16a2\xF8WPG\x91\x90PV[`@Q\x7Fp\xA0\x821\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R0`\x04\x82\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16\x90cp\xA0\x821\x90`$\x01` `@Q\x80\x83\x03\x81\x86Z\xFA\x15\x80\x15a3`W=_\x80>=_\xFD[PPPP`@Q=`\x1F\x19`\x1F\x82\x01\x16\x82\x01\x80`@RP\x81\x01\x90a\x07e\x91\x90ag\x88V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16\x7F'\xE0\x98\xC5\x05\xD4N\xC3W@\x04\xBC\xA0R\xAA\xBFv\xBD5\0L\x18 \x99\xD8\xC5u\xFB#\x85\x93\xB9]\x80\x7F\x1E\x07E\xA7\xDB\x16#\x98\x1F\x0B*]B26L\0xrf\xEBu\xADTo\x19\x0El\xEB\xE9\xBD\x95]PPV[` \x83\x01Q_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x163\x03a4\x10W_\x91Pa5\x80V[`\x80\x88\x16\x15a5\x80W_a4\xB2\x893\x8A\x8A\x8A\x8A`@Q`$\x01a47\x95\x94\x93\x92\x91\x90ag\x9FV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7FW^$\xB4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90Ra:KV[\x90P\x80Q``\x14a4\xE6Wa4\xE6\x7F\x1E\x04\x8E\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[`@\x88\x01Qb\xFF\xFF\xFF\x16b\x80\0\0\x03a5\x01W``\x81\x01Q\x91P[`\x08\x89\x16\x15a5~W`@\x81\x01Q\x92P_a5\x1C\x84`\x80\x1D\x90V[\x90P\x80`\x0F\x0B_\x14a5|W_\x85\x12a59`\x0F\x83\x90\x0B\x87ah\x87V[\x95P\x80a5HW_\x86\x12a5LV[_\x86\x13[\x15a5zWa5z\x7F\xFA\x0Bq\xD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[P[P[P[\x95P\x95P\x95\x92PPPV[_\x80\x80\x80\x80a5\x9A\x89\x88aF\xA9V[\x93P\x93P\x93P\x93P_\x83\x11\x15a5\xD5Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16_\x90\x81R`\x01` R`@\x90 \x80T\x84\x01\x90U[3\x88\x7F@\xE9\xCE\xCB\x9F_\x1F\x1C[\x9C\x97\xDE\xC2\x91{~\xE9.W\xBAUcp\x8D\xAC\xA9M\xD8J\xD7\x11/a6\x02\x87`\x80\x1D\x90V[a6\x0C\x88`\x0F\x0B\x90V[\x85Q`@\x80\x88\x01Q` \x80\x8A\x01Q\x83Q`\x0F\x97\x88\x0B\x81R\x95\x90\x96\x0B\x90\x85\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x90\x83\x01Ro\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16``\x82\x01R`\x02\x91\x90\x91\x0B`\x80\x82\x01Rb\xFF\xFF\xFF\x86\x16`\xA0\x82\x01R`\xC0\x01`@Q\x80\x91\x03\x90\xA3P\x91\x97\x96PPPPPPPV[_\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x163\x03a6\xB5WP\x84\x90P_a,\xF6V[_a6\xC0\x84`\x80\x1D\x90V[\x90P_a6\xCD\x85`\x0F\x0B\x90V[\x90P`@\x8B\x16\x15a7\xA0Wa7\x93a*\x103\x8C\x8C\x8C\x8C\x8C`@Q`$\x01a6\xF9\x96\x95\x94\x93\x92\x91\x90ah\xAEV[`@\x80Q\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x81\x84\x03\x01\x81R\x91\x90R` \x81\x01\x80Q{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x7F\xB4{/\xB1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x17\x90R`\x04\x8E\x16\x15\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8F\x16\x91\x90aB\xB6V[a7\x9D\x90\x82ai\x9DV[\x90P[_\x81`\x0F\x0B_\x14\x15\x80a7\xB6WP\x82`\x0F\x0B_\x14\x15[\x15a8\x13W\x89Q` \x8B\x01Q_\x13\x90\x15\x15\x14a7\xEAWo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16`\x80\x83\x90\x1B\x17a8\x04V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x80\x84\x90\x1B\x17[\x90Pa8\x10\x89\x82aC\x10V[\x98P[\x97\x9B\x97\x9AP\x96\x98PPPPPPPPPV[3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x81\x14\x80\x15\x90a8~WPs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16_\x90\x81R`\x03` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x90R T`\xFF\x16\x15[\x15a9+Ws\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x85\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x85\x16\x83R\x92\x81R\x82\x82 \x86\x83R\x90R T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x14a9)Wa8\xEE\x83\x82aa\x8DV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x87\x16_\x90\x81R`\x05` \x90\x81R`@\x80\x83 \x93\x87\x16\x83R\x92\x81R\x82\x82 \x88\x83R\x90R U[P[a\t\xA5\x84\x84\x84aO;V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x81\x16_\x90\x81R\x90\x84\x16` R`@\x81 \x80\\\x91\x90a9n`\x0F\x85\x90\x0B\x84ah\x87V[\x91P\x81\x81]P\x93P\x93\x91PPV[\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B\\`\x01\x81\x03\x90P\x80\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B]PV[\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B\\`\x01\x81\x01\x90P\x80\x7F}K1d\xC6\xE4[\x97\xE7\xD8{q%\xA4LX(\xD0\x05\xAF\x88\xF9\xD7Q\xCF\xD7\x87)\xC5\xD9\x9A\x0B]PV[=`@Q\x83\x81R\x82`\x04\x82\x01R`@`$\x82\x01R\x81`D\x82\x01R\x81_`d\x83\x01>` \x80`\x1F\x84\x01\x04\x02`d\x01\x91P\x81\x81\xFD[``_\x80_\x84Q` \x86\x01_\x88Z\xF1\x90P\x80a:\x8BWa:\x8B\x7F1\x9DT\xC3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x85a:\x18V[`@Q\x91P`\x1F\x19`?=\x01\x16\x82\x01`@R=\x82R=_` \x84\x01>` \x82Q\x10\x80a;\tWP` \x83\x01Q\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16a:\xE4\x83` \x01Q\x90V[\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x14\x15[\x15a;7Wa;7\x7F\x1E\x04\x8E\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[P\x92\x91PPV[\x80`\x02\x0B\x82`\x02\x0B\x12a;vWa;v\x7F\xC4C>\xD5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83\x83aO\xD1V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18`\x02\x83\x90\x0B\x12\x15a;\xCCWa;\xCC\x7F\xD5\xE2\xF7\xAB\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a-\x02V[b\r\x89\xE8`\x02\x82\x90\x0B\x13\x15a\x17\xC8Wa\x17\xC8\x7F\x1A\xD7w\xF8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82a-\x02V[`\x02\x83\x90\x0B_\x90\x81R`\x04\x85\x01` R`@\x81 \x80T\x82\x91\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x90p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\x0F\x0Ba<S\x82\x88aB\x86V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x82\x16\x15\x90\x84\x16\x80\x15\x91\x90\x91\x14\x15\x96P\x90\x94P_\x03a<\xA6W\x88T`\xA0\x1C`\x02\x0B`\x02\x0B\x88`\x02\x0B\x13a<\xA6W`\x01\x80\x8A\x01T\x90\x84\x01U`\x02\x80\x8A\x01T\x90\x84\x01U[_\x86a<\xBBWa<\xB6\x88\x83ai\x9DV[a<\xC5V[a<\xC5\x88\x83ai\xEBV[\x90P\x80`\x80\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x17\x84UPPPP\x94P\x94\x92PPPV[`\x02\x91\x82\x0B\x91\x0B\x80\x82\x07\x15a=\x1BW`@Qc\xD4\xD8\xF3\xE6\x81R\x82` \x82\x01R\x81`@\x82\x01R`D`\x1C\x82\x01\xFD[\x80\x82\x05\x91P\x81`\x08\x1D_R\x82` R`@_ `\x01`\xFF\x84\x16\x1B\x81T\x18\x81UPPPPV[`\x02\x82\x81\x0B_\x81\x81R`\x04\x86\x01` R`@\x80\x82 \x85\x85\x0B\x83R\x90\x82 \x87T\x92\x94\x85\x94\x92\x93\x91\x92`\xA0\x92\x90\x92\x1C\x90\x0B\x90\x81\x12\x15a=\x96W\x81`\x01\x01T\x83`\x01\x01T\x03\x94P\x81`\x02\x01T\x83`\x02\x01T\x03\x93Pa=\xE9V[\x85`\x02\x0B\x81`\x02\x0B\x12a=\xC2W\x82`\x01\x01T\x82`\x01\x01T\x03\x94P\x82`\x02\x01T\x82`\x02\x01T\x03\x93Pa=\xE9V[\x81`\x01\x01T\x83`\x01\x01T\x89`\x01\x01T\x03\x03\x94P\x81`\x02\x01T\x83`\x02\x01T\x89`\x02\x01T\x03\x03\x93P[PPP\x93P\x93\x91PPV[\x83T_\x90\x81\x90o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`\x0F\x86\x90\x0B\x82\x03a>^W\x80o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16_\x03a>YWa>Y\x7F\xAE\xFE\xB9$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[a>\xA5V[a>h\x81\x87aB\x86V[\x87T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x87U[a>\xD9\x87`\x01\x01T\x86\x03\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aO\xEEV[\x92Pa?\x0F\x87`\x02\x01T\x85\x03\x82o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0aO\xEEV[`\x01\x88\x01\x95\x90\x95UPP`\x02\x90\x94\x01U\x91\x92\x90PV[`\x02\x0B_`\xFF\x82\x90\x1D\x80\x83\x01\x18b\r\x89\xE8\x81\x11\x15a?gWa?g\x7F\x8B\x862z\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x84a-\x02V[p\x01\xFF\xFC\xB93\xBDo\xAD7\xAA-\x16-\x1AY@\x01`\x01\x82\x16\x02p\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x18`\x02\x82\x16\x15a?\xB0Wo\xFF\xF9rr7=A2Y\xA4i\x90X\x0E!:\x02`\x80\x1C[`\x04\x82\x16\x15a?\xCFWo\xFF\xF2\xE5\x0F_ei2\xEF\x125|\xF3\xC7\xFD\xCC\x02`\x80\x1C[`\x08\x82\x16\x15a?\xEEWo\xFF\xE5\xCA\xCA~\x10\xE4\xE6\x1C6$\xEA\xA0\x94\x1C\xD0\x02`\x80\x1C[`\x10\x82\x16\x15a@\rWo\xFF\xCB\x98C\xD6\x0FaY\xC9\xDBX\x83\\\x92fD\x02`\x80\x1C[` \x82\x16\x15a@,Wo\xFF\x97;A\xFA\x98\xC0\x81G.h\x96\xDF\xB2T\xC0\x02`\x80\x1C[`@\x82\x16\x15a@KWo\xFF.\xA1df\xC9j8C\xECx\xB3&\xB5(a\x02`\x80\x1C[`\x80\x82\x16\x15a@jWo\xFE]\xEE\x04j\x99\xA2\xA8\x11\xC4a\xF1\x96\x9C0S\x02`\x80\x1C[a\x01\0\x82\x16\x15a@\x8AWo\xFC\xBE\x86\xC7\x90\n\x88\xAE\xDC\xFF\xC8;G\x9A\xA3\xA4\x02`\x80\x1C[a\x02\0\x82\x16\x15a@\xAAWo\xF9\x87\xA7%:\xC4\x13\x17o+\x07L\xF7\x81^T\x02`\x80\x1C[a\x04\0\x82\x16\x15a@\xCAWo\xF39+\x08\"\xB7\0\x05\x94\x0Cz9\x8EKp\xF3\x02`\x80\x1C[a\x08\0\x82\x16\x15a@\xEAWo\xE7\x15\x94u\xA2\xC2\x9BtC\xB2\x9C\x7F\xA6\xE8\x89\xD9\x02`\x80\x1C[a\x10\0\x82\x16\x15aA\nWo\xD0\x97\xF3\xBD\xFD \"\xB8\x84Z\xD8\xF7\x92\xAAX%\x02`\x80\x1C[a \0\x82\x16\x15aA*Wo\xA9\xF7FF-\x87\x0F\xDF\x8Ae\xDC\x1F\x90\xE0a\xE5\x02`\x80\x1C[a@\0\x82\x16\x15aAJWop\xD8i\xA1V\xD2\xA1\xB8\x90\xBB=\xF6+\xAF2\xF7\x02`\x80\x1C[a\x80\0\x82\x16\x15aAjWo1\xBE\x13_\x97\xD0\x8F\xD9\x81#\x15\x05T/\xCF\xA6\x02`\x80\x1C[b\x01\0\0\x82\x16\x15aA\x8BWo\t\xAAP\x8B[z\x84\xE1\xC6w\xDET\xF3\xE9\x9B\xC9\x02`\x80\x1C[b\x02\0\0\x82\x16\x15aA\xABWn]j\xF8\xDE\xDB\x81\x19f\x99\xC3)\"^\xE6\x04\x02`\x80\x1C[b\x04\0\0\x82\x16\x15aA\xCAWm\"\x16\xE5\x84\xF5\xFA\x1E\xA9&\x04\x1B\xED\xFE\x98\x02`\x80\x1C[b\x08\0\0\x82\x16\x15aA\xE7Wk\x04\x8A\x17\x03\x91\xF7\xDCBDN\x8F\xA2\x02`\x80\x1C[_\x84\x13\x15aB\x12W\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x04[c\xFF\xFF\xFF\xFF\x01` \x1C\x93\x92PPPV[_\x80\x82`\x0F\x0B\x12aBIWaBBaB=\x85\x85\x85`\x01aP\xA9V[aQ\xDBV[_\x03a\x1D\x16V[a\x1D\x16aB=\x85\x85\x85_\x03_aP\xA9V[_\x80\x82`\x0F\x0B\x12aBuWaBBaB=\x85\x85\x85`\x01aR\rV[a\x1D\x16aB=\x85\x85\x85_\x03_aR\rV[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x16`\x0F\x82\x90\x0B\x01`\x80\x81\x90\x1C\x15a\x07eWc\x93\xDA\xFD\xF1_R`\x04`\x1C\xFD[_\x80aB\xC2\x85\x85a:KV[\x90P\x82aB\xD2W_\x91PPa\x088V[\x80Q`@\x14aC\x04WaC\x04\x7F\x1E\x04\x8E\x1D\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[`@\x01Q\x94\x93PPPPV[_`\x80\x82\x81\x1D\x90\x84\x90\x1D\x03`\x0F\x83\x81\x0B\x90\x85\x90\x0B\x03a+\"a+\x19\x83a&~V[_a'\x10\x82\x11\x15aCnW`@Q\x7F\xDE\xAA\x01\xE6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81R`\x04\x01`@Q\x80\x91\x03\x90\xFD[a'\x10aC{\x83\x85aj9V[a\x088\x91\x90aj}V[_s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DQc\x96\x16\x82\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFE\xFF\xFD\x89]\x83\x01s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aD\x04WaD\x04\x7FaHu$\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x83a.\"V[w\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0` \x83\x90\x1B\x16\x80_aD.\x82aRxV[`\xFF\x16\x90P`\x80\x81\x10aDIW`\x7F\x81\x03\x83\x90\x1C\x91PaDSV[\x80`\x7F\x03\x83\x90\x1B\x91P[\x90\x80\x02`\x7F\x81\x81\x1C`\xFF\x83\x81\x1C\x91\x90\x91\x1C\x80\x02\x80\x83\x1C\x81\x83\x1C\x1C\x80\x02\x80\x84\x1C\x81\x84\x1C\x1C\x80\x02\x80\x85\x1C\x81\x85\x1C\x1C\x80\x02\x80\x86\x1C\x81\x86\x1C\x1C\x80\x02\x80\x87\x1C\x81\x87\x1C\x1C\x80\x02\x80\x88\x1C\x81\x88\x1C\x1C\x80\x02\x80\x89\x1C\x81\x89\x1C\x1C\x80\x02\x80\x8A\x1C\x81\x8A\x1C\x1C\x80\x02\x80\x8B\x1C\x81\x8B\x1C\x1C\x80\x02\x80\x8C\x1C\x81\x8C\x1C\x1C\x80\x02\x80\x8D\x1C\x81\x8D\x1C\x1C\x80\x02\x80\x8E\x1C\x9C\x81\x90\x1C\x9C\x90\x9C\x1C\x80\x02\x9C\x8D\x90\x1C\x9E\x9D\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8F\x01`@\x1B`\xC0\x91\x90\x91\x1Cg\x80\0\0\0\0\0\0\0\x16\x17`\xC1\x9B\x90\x9B\x1Cg@\0\0\0\0\0\0\0\x16\x9A\x90\x9A\x17`\xC2\x99\x90\x99\x1Cg \0\0\0\0\0\0\0\x16\x98\x90\x98\x17`\xC3\x97\x90\x97\x1Cg\x10\0\0\0\0\0\0\0\x16\x96\x90\x96\x17`\xC4\x95\x90\x95\x1Cg\x08\0\0\0\0\0\0\0\x16\x94\x90\x94\x17`\xC5\x93\x90\x93\x1Cg\x04\0\0\0\0\0\0\0\x16\x92\x90\x92\x17`\xC6\x91\x90\x91\x1Cg\x02\0\0\0\0\0\0\0\x16\x17`\xC7\x91\x90\x91\x1Cg\x01\0\0\0\0\0\0\0\x16\x17`\xC8\x91\x90\x91\x1Cf\x80\0\0\0\0\0\0\x16\x17`\xC9\x91\x90\x91\x1Cf@\0\0\0\0\0\0\x16\x17`\xCA\x91\x90\x91\x1Cf \0\0\0\0\0\0\x16\x17`\xCB\x91\x90\x91\x1Cf\x10\0\0\0\0\0\0\x16\x17`\xCC\x91\x90\x91\x1Cf\x08\0\0\0\0\0\0\x16\x17`\xCD\x91\x90\x91\x1Cf\x04\0\0\0\0\0\0\x16\x17i6'\xA3\x01\xD7\x10UwL\x85\x81\x02\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFDp\x9B~T\x80\xFB\xA5\xA5\x0F\xED^b\xFF\xC5V\x81\x01`\x80\x90\x81\x1D\x90o\xDB-\xF0\x9E\x81\x95\x9A\x81E^&\x07\x99\xA0c/\x83\x01\x90\x1D`\x02\x81\x81\x0B\x90\x83\x90\x0B\x14aF\x9AW\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aFr\x82a?%V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aF\x94W\x81aF\x9CV[\x80aF\x9CV[\x81[\x99\x98PPPPPPPPPV[`@\x80Q``\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R\x81\x90\x81\x90\x85T`@\x86\x01Q_\x81aF\xE3Wa\x0F\xFF`\xC4\x84\x90\x1C\x16aF\xEDV[a\x0F\xFF`\xB8\x84\x90\x1C\x16[\x88Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16\x86Ra\xFF\xFF\x91\x90\x91\x16\x91P_`\xA0\x85\x90\x1C`\x02\x0B`\x02\x0B` \x87\x01R`\x03\x8B\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x87\x01R`\x80\x8A\x01Q_\x90b@\0\0\x16aG]W`\xD0\x86\x90\x1Cb\xFF\xFF\xFF\x16aGoV[aGo\x8B`\x80\x01Qb\xFF\xFF\xFF\x16aS\x0CV[\x90P\x83\x15aG\x9DWb\x0FB@a\x0F\xFF\x85\x16b\xFF\xFF\xFF\x83\x16\x81\x81\x02\x83\x81\x06\x15\x15\x93\x90\x04\x92\x90\x92\x01\x91\x01\x03aG\x9FV[\x80[\x97PPb\x0FB@\x87b\xFF\xFF\xFF\x16\x10aG\xE3W\x89Q_\x12\x15aG\xE3WaG\xE3\x7F\x96 bF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[\x89Q_\x03aG\xFBW_\x80\x98P\x98PPPPPPaO2V[\x83\x15aH\xDEW``\x8A\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x16\x10aHmWaHms\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16[``\x8C\x01Q\x7F|\x9Cn\x8F\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x91\x90a-\x11V[d\x01\0\x02v\xA3s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8A``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11aH\xD9W``\x8A\x01QaH\xD9\x90\x7F\x9EM|\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a.\"V[aI\x9CV[``\x8A\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x81\x16\x91\x16\x11aI!WaI!s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16aH@V[s\xFF\xFD\x89c\xEF\xD1\xFCjPd\x88I]\x95\x1DRc\x98\x8D&s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x8A``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x10aI\x9CW``\x8A\x01QaI\x9C\x90\x7F\x9EM|\xC7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90a.\"V[`@\x80Qa\x01\0\x81\x01\x82R_\x80\x82R` \x82\x01\x81\x90R\x91\x81\x01\x82\x90R``\x81\x01\x82\x90R`\x80\x81\x01\x82\x90R`\xA0\x81\x01\x82\x90R`\xC0\x81\x01\x82\x90R`\xE0\x81\x01\x91\x90\x91R\x84aI\xEBW\x8B`\x02\x01TaI\xF1V[\x8B`\x01\x01T[`\xE0\x82\x01R[\x82\x15\x80aJ6WP\x8A``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x14[aM\xBEW\x86Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x81R` \x80\x88\x01Q\x90\x8C\x01QaJm\x91`\x05\x8F\x01\x91\x88aS\x1BV[\x15\x15`@\x83\x01R`\x02\x0B` \x82\x01\x81\x90R\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18\x12aJ\xCBW\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xF2v\x18` \x82\x01R[b\r\x89\xE8`\x02\x0B\x81` \x01Q`\x02\x0B\x12aJ\xE9Wb\r\x89\xE8` \x82\x01R[aJ\xF6\x81` \x01Qa?%V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16``\x83\x81\x01\x82\x90R\x89Q\x90\x8E\x01QaK@\x93\x91\x92\x91\x16\x80\x82\x18\x91\x81\x11`\x01\x8A\x16\x18\x91\x90\x91\x02\x18\x89`@\x01Q\x86\x8CaTFV[`\xC0\x85\x01R`\xA0\x84\x01R`\x80\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87R\x8AQ_\x12\x15aK\xA9WaK}\x81`\xA0\x01QaQ\xDBV[\x83\x03\x92PaK\x98\x81`\xC0\x01Q\x82`\x80\x01QaB=\x91\x90aa\xA0V[aK\xA2\x90\x83aj\xB5V[\x91PaK\xDAV[aK\xBC\x81`\xC0\x01Q\x82`\x80\x01Q\x01aQ\xDBV[\x83\x01\x92PaK\xCD\x81`\xA0\x01QaQ\xDBV[aK\xD7\x90\x83ah\x87V[\x91P[\x83\x15aL\x16W_b\x0FB@\x85\x83`\xC0\x01Q\x84`\x80\x01Q\x01\x02\x81aK\xFFWaK\xFFajPV[`\xC0\x84\x01\x80Q\x92\x90\x91\x04\x91\x82\x90\x03\x90R\x99\x90\x99\x01\x98P[`@\x87\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x15aLuWaLi\x81`\xC0\x01Qp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x89`@\x01Qo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x91\x02\x04\x90V[`\xE0\x82\x01\x80Q\x90\x91\x01\x90R[\x80``\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x87_\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x03aM\x8BW\x80`@\x01Q\x15aMfW_\x80\x86aL\xCFW\x8D`\x01\x01T\x83`\xE0\x01QaL\xDAV[\x82`\xE0\x01Q\x8E`\x02\x01T[\x91P\x91P_aM2\x8F\x85` \x01Q\x85\x85`\x02\x92\x83\x0B_\x90\x81R`\x04\x90\x94\x01` R`@\x90\x93 `\x01\x81\x01\x80T\x90\x92\x03\x90\x91U\x90\x81\x01\x80T\x90\x92\x03\x90\x91UTp\x01\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x90\x04`\x0F\x0B\x90V[\x90P\x87\x15aM=W_\x03[aMK\x8A`@\x01Q\x82aB\x86V[o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16`@\x8B\x01RPPP[\x84aMuW\x80` \x01QaM~V[`\x01\x81` \x01Q\x03[`\x02\x0B` \x88\x01RaI\xF7V[\x80Q\x87Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14aM\xB9W\x86QaM~\x90aC\x85V[aI\xF7V[\x86Q` \x88\x01QaNS\x91\x90aN\x15\x90\x89\x90`\xA0\x1Bv\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x91\x90\x91\x16\x17\x90V[\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x91\x16\x17\x90V[\x8CU`@\x87\x01Q`\x03\x8D\x01To\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x81\x16\x91\x16\x14aN\xC2W`@\x87\x01Q`\x03\x8D\x01\x80T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x16o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x90\x92\x16\x91\x90\x91\x17\x90U[\x84aN\xD6W`\xE0\x81\x01Q`\x02\x8D\x01UaN\xE1V[`\xE0\x81\x01Q`\x01\x8D\x01U[\x8AQ_\x13\x85\x15\x15\x14aO\x0EWaO\x07aN\xF9\x83a&~V[a)%\x85\x8E_\x01Q\x03a&~V[\x99PaO+V[aO(aO\x1F\x84\x8D_\x01Q\x03a&~V[a)%\x84a&~V[\x99P[PPPPPP[\x92\x95\x91\x94P\x92PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x83\x16_\x90\x81R`\x04` \x90\x81R`@\x80\x83 \x85\x84R\x90\x91R\x81 \x80T\x83\x92\x90aOz\x90\x84\x90aa\x8DV[\x90\x91UPP`@\x80Q3\x81R` \x81\x01\x83\x90R\x83\x91_\x91s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x91\x7F\x1B=~\xDB.\x9C\x0B\x0E|R[ \xAA\xAE\xF0\xF5\x94\r.\xD7\x16c\xC7\xD3\x92f\xEC\xAF\xACr\x88Y\x91\x01a AV[`@Q\x83\x81R\x82`\x02\x0B`\x04\x82\x01R\x81`\x02\x0B`$\x82\x01R`D\x81\xFD[_\x83\x83\x02\x81\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x87\t\x82\x81\x10\x83\x82\x03\x03\x91PP\x80\x84\x11aP,W_\x80\xFD[\x80_\x03aP>WP\x82\x90\x04\x90Pa\x088V[_\x84\x86\x88\t_\x86\x81\x03\x87\x16\x96\x87\x90\x04\x96`\x02`\x03\x89\x02\x81\x18\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x82\x03\x02\x80\x8A\x02\x90\x91\x03\x02\x91\x81\x90\x03\x81\x90\x04`\x01\x01\x86\x84\x11\x90\x95\x03\x94\x90\x94\x02\x91\x90\x94\x03\x92\x90\x92\x04\x91\x90\x91\x17\x91\x90\x91\x02\x91PP\x93\x92PPPV[_\x83s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x85s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x11\x15aP\xE2W\x92\x93\x92[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16aQ\tWb\xBF\xC9!_R`\x04`\x1C\xFD[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x84\x90\x1B\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x86\x03\x16\x83aQ\x95W\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aQ\x82\x83\x83\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aO\xEEV[\x81aQ\x8FWaQ\x8FajPV[\x04a0\x9DV[a0\x9DaQ\xB9\x83\x83\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aU\xB6V[\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x80\x82\x04\x91\x06\x15\x15\x01\x90V[\x80_\x81\x12\x15a\x0CbWa\x0Cb\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x81\x16\x90\x86\x16\x03`\xFF\x81\x90\x1D\x90\x81\x01\x18l\x01\0\0\0\0\0\0\0\0\0\0\0\0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16aR_\x81\x84\x84aO\xEEV[\x93P\x84_\x83\x85\x84\t\x11\x16\x84\x01\x93PPPP\x94\x93PPPPV[_\x80\x82\x11aR\x84W_\x80\xFD[P\x7F\x07\x06\x06\x05\x06\x02\x05\0\x06\x02\x03\x02\x05\x04\0\x01\x06\x05\x02\x05\x03\x03\x04\x01\x05\x05\x03\x04\0\0\0\0`\x1Fo\x84!\x08B\x10\x84!\x08\xCCc\x18\xC6\xDBmT\xBEo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11`\x07\x1B\x84\x81\x1Cg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x10`\x06\x1B\x17\x84\x81\x1Cc\xFF\xFF\xFF\xFF\x10`\x05\x1B\x17\x84\x81\x1Ca\xFF\xFF\x10`\x04\x1B\x17\x84\x81\x1C`\xFF\x10`\x03\x1B\x17\x93\x84\x1C\x1C\x16\x1A\x17\x90V[b\xBF\xFF\xFF\x81\x16a\x0Cb\x81a$%V[_\x80`\x02\x84\x81\x0B\x90\x86\x90\x0B\x81\x81\x07\x83\x13\x91\x90\x05\x03\x83\x15aS\xB9W`\x02\x81\x90\x0B`\x08\x1D`\x01\x81\x90\x0B_\x90\x81R` \x89\x90R`@\x90 T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xFF\x80\x85\x16\x90\x81\x90\x03\x91\x90\x91\x1C\x91\x82\x16\x80\x15\x15\x95P\x90\x91\x90\x85aS\x9BW\x88\x83`\xFF\x16\x86\x03\x02aS\xAEV[\x88aS\xA5\x82aRxV[\x84\x03`\xFF\x16\x86\x03\x02[\x96PPPPPaT<V[`\x01\x90\x81\x01`\x02\x81\x90\x0B`\x08\x1D\x80\x83\x0B_\x90\x81R` \x8A\x90R`@\x90 T\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\xFF\x84\x16\x94\x85\x1B\x01\x19\x90\x81\x16\x80\x15\x15\x95P\x92\x93\x91\x92\x85aT\"W\x88\x83`\xFF\x03`\xFF\x16\x86\x01\x02aT5V[\x88\x83aT-\x83aU\xE6V[\x03`\xFF\x16\x86\x01\x02[\x96PPPPP[P\x94P\x94\x92PPPV[_\x80\x80\x80b\xFF\xFF\xFF\x85\x16s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\x8A\x16\x90\x8B\x16\x10\x15\x82\x88\x12\x80\x15aU)W_aT\x8C\x8A_\x03\x85b\x0FB@\x03b\x0FB@aO\xEEV[\x90P\x82aT\xA5WaT\xA0\x8D\x8D\x8D`\x01aR\rV[aT\xB2V[aT\xB2\x8C\x8E\x8D`\x01aP\xA9V[\x96P\x86\x81\x10aT\xE6W\x8B\x97Pb\x0FB@\x84\x14aT\xDDWaT\xD8\x87\x85\x86b\x0FB@\x03aU\xB6V[aT\xDFV[\x86[\x94PaT\xFFV[\x80\x96PaT\xF5\x8D\x8C\x83\x86aV\x80V[\x97P\x86\x8A_\x03\x03\x94P[\x82aU\x15WaU\x10\x8D\x89\x8D_aP\xA9V[aU!V[aU!\x88\x8E\x8D_aR\rV[\x95PPaU\xA7V[\x81aU?WaU:\x8C\x8C\x8C_aP\xA9V[aUKV[aUK\x8B\x8D\x8C_aR\rV[\x94P\x84\x89\x10aU\\W\x8A\x96PaUnV[\x88\x94PaUk\x8C\x8B\x87\x85aV\xE4V[\x96P[\x81aU\x85WaU\x80\x8C\x88\x8C`\x01aR\rV[aU\x92V[aU\x92\x87\x8D\x8C`\x01aP\xA9V[\x95PaU\xA4\x86\x84\x85b\x0FB@\x03aU\xB6V[\x93P[PPP\x95P\x95P\x95P\x95\x91PPV[_aU\xC2\x84\x84\x84aO\xEEV[\x90P\x81\x80aU\xD2WaU\xD2ajPV[\x83\x85\t\x15a\x088W`\x01\x01\x80a\x088W_\x80\xFD[_\x80\x82\x11aU\xF2W_\x80\xFD[P~\x1F\r\x1E\x10\x0C\x1D\x07\x0F\t\x0B\x19\x13\x1C\x17\x06\x01\x0E\x11\x08\n\x1A\x14\x18\x02\x12\x1B\x15\x03\x16\x04\x05_\x82\x90\x03\x90\x91\x16a\x01\xE0\x7F\x80@@UC\0RfD2\0\0P a\x06t\x050&\x02\0\0\x10u\x06 \x01v\x11pw`\xFC\x7F\xB6\xDBm\xB6\xDD\xDD\xDD\xDD\xD3M4\xD3I$\x92I!\x08B\x10\x8Cc\x18\xC69\xCEs\x9C\xFF\xFF\xFF\xFF\x84\x02`\xF8\x1C\x16\x1B`\xF7\x1C\x16\x90\x81\x1Cc\xD7dS\xE0\x04`\x1F\x16\x91\x90\x91\x1A\x17\x90V[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x15\x17\x15aV\xC0WcO$a\xB8_R`\x04`\x1C\xFD[\x81aV\xD7WaV\xD2\x85\x85\x85`\x01aW=V[a+\"V[a+\"\x85\x85\x85`\x01aX\x9FV[_o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16\x15s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x15\x17\x15aW$WcO$a\xB8_R`\x04`\x1C\xFD[\x81aW5WaV\xD2\x85\x85\x85_aX\x9FV[a+\"\x85\x85\x85_[_\x81\x15aW\xE2W_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15aW\x90WaW\x8B\x84l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x87o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aO\xEEV[aW\xB0V[aW\xB0o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16``\x86\x90\x1Baj}V[\x90PaW\xDAaW\xD5\x82s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x89\x16aa\xA0V[aY\xD4V[\x91PPa\x1D\x16V[_s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x11\x15aX.WaX)\x84l\x01\0\0\0\0\0\0\0\0\0\0\0\0\x87o\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16aU\xB6V[aXTV[aXT``\x85\x90\x1Bo\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x80\x82\x04\x91\x06\x15\x15\x01\x90V[\x90P\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x11aX\x80WcC#\xA5U_R`\x04`\x1C\xFD[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x03\x90Pa\x1D\x16V[_\x82_\x03aX\xAEWP\x83a\x1D\x16V[{\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0``\x85\x90\x1B\x16\x82\x15aYyWs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84\x81\x02\x90\x85\x82\x81aY\x01WaY\x01ajPV[\x04\x03aY>W\x81\x81\x01\x82\x81\x10aY<WaY2\x83\x89s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16\x83aU\xB6V[\x93PPPPa\x1D\x16V[P[PaW\xDA\x81\x85aYds\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8A\x16\x83aj}V[aYn\x91\x90aa\xA0V[\x80\x82\x04\x91\x06\x15\x15\x01\x90V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x84\x81\x02\x90\x85\x82\x04\x14\x81\x83\x11\x16aY\xADWc\xF5\xC7\x87\xF1_R`\x04`\x1C\xFD[\x80\x82\x03aY2aW\xD5\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x8B\x16\x84aU\xB6V[\x80s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x0CbWa\x0Cb\x7F\x93\xDA\xFD\xF1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0a\x1D\x1EV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x16\x81\x14a\x17\x99W_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15aZMW_\x80\xFD[\x825aZX\x81aZ\x1BV[\x94` \x93\x90\x93\x015\x93PPPV[_` \x82\x84\x03\x12\x15aZvW_\x80\xFD[\x815\x7F\xFF\xFF\xFF\xFF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x16\x81\x14a\x088W_\x80\xFD[_\x80_``\x84\x86\x03\x12\x15aZ\xB7W_\x80\xFD[\x835aZ\xC2\x81aZ\x1BV[\x95` \x85\x015\x95P`@\x90\x94\x015\x93\x92PPPV[_\x80_``\x84\x86\x03\x12\x15aZ\xE9W_\x80\xFD[\x835aZ\xF4\x81aZ\x1BV[\x92P` \x84\x015a[\x04\x81aZ\x1BV[\x92\x95\x92\x94PPP`@\x91\x90\x91\x015\x90V[_` \x82\x84\x03\x12\x15a[%W_\x80\xFD[P5\x91\x90PV[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`A`\x04R`$_\xFD[`@Q`\x80\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a[|Wa[|a[,V[`@R\x90V[`@Q`\x1F\x82\x01\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0\x16\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a[\xC9Wa[\xC9a[,V[`@R\x91\x90PV[\x805b\xFF\xFF\xFF\x81\x16\x81\x14a\x0CbW_\x80\xFD[\x805`\x02\x81\x90\x0B\x81\x14a\x0CbW_\x80\xFD[_`\xA0\x82\x84\x03\x12\x15a\\\x04W_\x80\xFD[`@Q`\xA0\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a\\'Wa\\'a[,V[`@R\x90P\x80\x825a\\8\x81aZ\x1BV[\x81R` \x83\x015a\\H\x81aZ\x1BV[` \x82\x01Ra\\Y`@\x84\x01a[\xD1V[`@\x82\x01Ra\\j``\x84\x01a[\xE3V[``\x82\x01R`\x80\x83\x015a\\}\x81aZ\x1BV[`\x80\x91\x90\x91\x01R\x92\x91PPV[_\x80\x83`\x1F\x84\x01\x12a\\\x9AW_\x80\xFD[P\x815g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a\\\xB1W_\x80\xFD[` \x83\x01\x91P\x83` \x82\x85\x01\x01\x11\x15a\\\xC8W_\x80\xFD[\x92P\x92\x90PV[_\x80_\x80_a\x01\0\x86\x88\x03\x12\x15a\\\xE4W_\x80\xFD[a\\\xEE\x87\x87a[\xF4V[\x94P`\xA0\x86\x015\x93P`\xC0\x86\x015\x92P`\xE0\x86\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\x17W_\x80\xFD[a]#\x88\x82\x89\x01a\\\x8AV[\x96\x99\x95\x98P\x93\x96P\x92\x94\x93\x92PPPV[_` \x82\x84\x03\x12\x15a]DW_\x80\xFD[\x815a\x088\x81aZ\x1BV[_\x80`@\x83\x85\x03\x12\x15a]`W_\x80\xFD[PP\x805\x92` \x90\x91\x015\x91PV[` \x80\x82R\x82Q\x82\x82\x01\x81\x90R_\x91\x84\x01\x90`@\x84\x01\x90\x83[\x81\x81\x10\x15a]\xA6W\x83Q\x83R` \x93\x84\x01\x93\x90\x92\x01\x91`\x01\x01a]\x88V[P\x90\x95\x94PPPPPV[_\x80` \x83\x85\x03\x12\x15a]\xC2W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a]\xD8W_\x80\xFD[a]\xE4\x85\x82\x86\x01a\\\x8AV[\x90\x96\x90\x95P\x93PPPPV[` \x81R_\x82Q\x80` \x84\x01R\x80` \x85\x01`@\x85\x01^_`@\x82\x85\x01\x01R`@\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x83\x01\x16\x84\x01\x01\x91PP\x92\x91PPV[_\x80`\xC0\x83\x85\x03\x12\x15a^TW_\x80\xFD[a^^\x84\x84a[\xF4V[\x91Pa^l`\xA0\x84\x01a[\xD1V[\x90P\x92P\x92\x90PV[\x805\x80\x15\x15\x81\x14a\x0CbW_\x80\xFD[_\x80`@\x83\x85\x03\x12\x15a^\x95W_\x80\xFD[\x825a^\xA0\x81aZ\x1BV[\x91Pa^l` \x84\x01a^uV[_\x80_\x80\x84\x86\x03a\x01@\x81\x12\x15a^\xC3W_\x80\xFD[a^\xCD\x87\x87a[\xF4V[\x94P`\x80\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x82\x01\x12\x15a^\xFEW_\x80\xFD[Pa_\x07a[YV[a_\x13`\xA0\x87\x01a[\xE3V[\x81Ra_!`\xC0\x87\x01a[\xE3V[` \x82\x01R`\xE0\x86\x015`@\x82\x01Ra\x01\0\x86\x015``\x82\x01R\x92Pa\x01 \x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a_WW_\x80\xFD[a_c\x87\x82\x88\x01a\\\x8AV[\x95\x98\x94\x97P\x95PPPPV[_\x80_\x80`\xE0\x85\x87\x03\x12\x15a_\x82W_\x80\xFD[a_\x8C\x86\x86a[\xF4V[\x93P`\xA0\x85\x015a_\x9C\x81aZ\x1BV[\x92P`\xC0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a_WW_\x80\xFD[_\x80` \x83\x85\x03\x12\x15a_\xC8W_\x80\xFD[\x825g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a_\xDEW_\x80\xFD[\x83\x01`\x1F\x81\x01\x85\x13a_\xEEW_\x80\xFD[\x805g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a`\x04W_\x80\xFD[\x85` \x82`\x05\x1B\x84\x01\x01\x11\x15a`\x18W_\x80\xFD[` \x91\x90\x91\x01\x95\x90\x94P\x92PPPV[_\x80`@\x83\x85\x03\x12\x15a`9W_\x80\xFD[\x825a`D\x81aZ\x1BV[\x91P` \x83\x015a`T\x81aZ\x1BV[\x80\x91PP\x92P\x92\x90PV[_\x80_\x80\x84\x86\x03a\x01 \x81\x12\x15a`tW_\x80\xFD[a`~\x87\x87a[\xF4V[\x94P``\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x82\x01\x12\x15a`\xAFW_\x80\xFD[P`@Q``\x81\x01g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x82\x82\x10\x17\x15a`\xD3Wa`\xD3a[,V[`@Ra`\xE2`\xA0\x87\x01a^uV[\x81R`\xC0\x86\x015` \x82\x01R`\xE0\x86\x015a`\xFC\x81aZ\x1BV[`@\x82\x01R\x92Pa\x01\0\x85\x015g\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15a_WW_\x80\xFD[_\x80_\x80`\x80\x85\x87\x03\x12\x15aa0W_\x80\xFD[\x845aa;\x81aZ\x1BV[\x93P` \x85\x015aaK\x81aZ\x1BV[\x93\x96\x93\x95PPPP`@\x82\x015\x91``\x015\x90V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x11`\x04R`$_\xFD[\x81\x81\x03\x81\x81\x11\x15a\x07eWa\x07eaa`V[\x80\x82\x01\x80\x82\x11\x15a\x07eWa\x07eaa`V[\x81\x83R\x81\x81` \x85\x017P_` \x82\x84\x01\x01R_` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x84\x01\x01\x90P\x92\x91PPV[` \x81R_a\x1D\x16` \x83\x01\x84\x86aa\xB3V[_` \x82\x84\x03\x12\x15ab\x1DW_\x80\xFD[\x81Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ab3W_\x80\xFD[\x82\x01`\x1F\x81\x01\x84\x13abCW_\x80\xFD[\x80Qg\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x11\x15ab]Wab]a[,V[ab\x8E` \x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xE0`\x1F\x84\x01\x16\x01a[\x82V[\x81\x81R\x85` \x83\x85\x01\x01\x11\x15ab\xA2W_\x80\xFD[\x81` \x84\x01` \x83\x01^_\x91\x81\x01` \x01\x91\x90\x91R\x94\x93PPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81RacZ` \x82\x01\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x84`\xC0\x82\x01R\x83`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01R_ac\x7Fa\x01 \x83\x01\x84\x86aa\xB3V[\x98\x97PPPPPPPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81Rad&` \x82\x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x83Q`\x02\x90\x81\x0B`\xC0\x83\x01R` \x85\x01Q\x90\x0B`\xE0\x82\x01R`@\x84\x01Qa\x01\0\x82\x01R``\x84\x01Qa\x01 \x82\x01Ra\x01`a\x01@\x82\x01R_a0\x9Da\x01`\x83\x01\x84\x86aa\xB3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x88\x16\x81Rae\x08` \x82\x01\x88s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x85Q`\x02\x90\x81\x0B`\xC0\x83\x01R` \x87\x01Q\x90\x0B`\xE0\x82\x01R`@\x86\x01Qa\x01\0\x82\x01R``\x86\x01Qa\x01 \x82\x01R\x84a\x01@\x82\x01R\x83a\x01`\x82\x01Ra\x01\xA0a\x01\x80\x82\x01R_aF\x9Ca\x01\xA0\x83\x01\x84\x86aa\xB3V[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81Rae\xF8` \x82\x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x84\x16`\xC0\x82\x01Ra\x01\0`\xE0\x82\x01R_a0\x9Da\x01\0\x83\x01\x84\x86aa\xB3V[`\xA0\x81\x01a\x07e\x82\x84s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81RagJ` \x82\x01\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x85\x16`\xC0\x82\x01R\x83`\x02\x0B`\xE0\x82\x01Ra\x01 a\x01\0\x82\x01R_ac\x7Fa\x01 \x83\x01\x84\x86aa\xB3V[_` \x82\x84\x03\x12\x15ag\x98W_\x80\xFD[PQ\x91\x90PV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x86\x16\x81Rah:` \x82\x01\x86s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x83Q\x15\x15`\xC0\x82\x01R` \x84\x01Q`\xE0\x82\x01R`@\x84\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x82\x01Ra\x01@a\x01 \x82\x01R_a0\x9Da\x01@\x83\x01\x84\x86aa\xB3V[\x80\x82\x01\x82\x81\x12_\x83\x12\x80\x15\x82\x16\x82\x15\x82\x16\x17\x15ah\xA6Wah\xA6aa`V[PP\x92\x91PPV[s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x87\x16\x81RaiI` \x82\x01\x87s\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81Q\x16\x82Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF` \x82\x01Q\x16` \x83\x01Rb\xFF\xFF\xFF`@\x82\x01Q\x16`@\x83\x01R``\x81\x01Q`\x02\x0B``\x83\x01Rs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF`\x80\x82\x01Q\x16`\x80\x83\x01RPPV[\x84Q\x15\x15`\xC0\x82\x01R` \x85\x01Q`\xE0\x82\x01R`@\x85\x01Qs\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x16a\x01\0\x82\x01R\x83a\x01 \x82\x01Ra\x01`a\x01@\x82\x01R_ac\x7Fa\x01`\x83\x01\x84\x86aa\xB3V[`\x0F\x81\x81\x0B\x90\x83\x90\x0B\x01o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x81\x13\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x82\x12\x17\x15a\x07eWa\x07eaa`V[`\x0F\x82\x81\x0B\x90\x82\x90\x0B\x03\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x80\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\x81\x12o\x7F\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\xFF\x82\x13\x17\x15a\x07eWa\x07eaa`V[\x80\x82\x02\x81\x15\x82\x82\x04\x84\x14\x17a\x07eWa\x07eaa`V[\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[_\x82aj\xB0W\x7FNH{q\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0_R`\x12`\x04R`$_\xFD[P\x04\x90V[\x81\x81\x03_\x83\x12\x80\x15\x83\x83\x13\x16\x83\x83\x12\x82\x16\x17\x15a;7Wa;7aa`V\xFE\xA1dsolcC\0\x08\x1A\0\n",
    );
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct BalanceDelta(alloy::sol_types::private::primitives::aliases::I256);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<BalanceDelta>
        for alloy::sol_types::private::primitives::aliases::I256 {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Int<256>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl BalanceDelta {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(
                value: alloy::sol_types::private::primitives::aliases::I256,
            ) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(
                self,
            ) -> alloy::sol_types::private::primitives::aliases::I256 {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for BalanceDelta {
            type RustType = alloy::sol_types::private::primitives::aliases::I256;
            type Token<'a> = <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Int<
                256,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for BalanceDelta {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Int<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct Currency(alloy::sol_types::private::Address);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Currency>
        for alloy::sol_types::private::Address {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::Token<
                '_,
            > {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::Address,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        self,
                    )
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::abi_encode_packed_to(
                    self,
                    out,
                )
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::abi_encoded_size(
                    self,
                )
            }
        }
        #[automatically_derived]
        impl Currency {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::Address) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::Address {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for Currency {
            type RustType = alloy::sol_types::private::Address;
            type Token<'a> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::Token<
                'a,
            >;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::type_check(
                    token,
                )
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::detokenize(
                    token,
                )
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for Currency {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                    rust,
                )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    out,
                )
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    rust,
                )
            }
        }
    };
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct PoolId(alloy::sol_types::private::FixedBytes<32>);
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<PoolId>
        for alloy::sol_types::private::FixedBytes<32> {
            #[inline]
            fn stv_to_tokens(
                &self,
            ) -> <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'_> {
                alloy_sol_types::private::SolTypeValue::<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >::stv_to_tokens(self)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::tokenize(self)
                    .0
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(self, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::abi_encoded_size(self)
            }
        }
        #[automatically_derived]
        impl PoolId {
            /// The Solidity type name.
            pub const NAME: &'static str = stringify!(@ name);
            /// Convert from the underlying value type.
            #[inline]
            pub const fn from(value: alloy::sol_types::private::FixedBytes<32>) -> Self {
                Self(value)
            }
            /// Return the underlying value.
            #[inline]
            pub const fn into(self) -> alloy::sol_types::private::FixedBytes<32> {
                self.0
            }
            /// Return the single encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode(&self.0)
            }
            /// Return the packed encoding of this value, delegating to the
            /// underlying type.
            #[inline]
            pub fn abi_encode_packed(&self) -> alloy_sol_types::private::Vec<u8> {
                <Self as alloy_sol_types::SolType>::abi_encode_packed(&self.0)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for PoolId {
            type RustType = alloy::sol_types::private::FixedBytes<32>;
            type Token<'a> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = Self::NAME;
            const ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <alloy::sol_types::sol_data::FixedBytes<
                32,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                Self::type_check(token).is_ok()
            }
            #[inline]
            fn type_check(token: &Self::Token<'_>) -> alloy_sol_types::Result<()> {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::type_check(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::SolType>::detokenize(token)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PoolId {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::topic_preimage_length(rust)
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(rust, out)
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                <alloy::sol_types::sol_data::FixedBytes<
                    32,
                > as alloy_sol_types::EventTopic>::encode_topic(rust)
            }
        }
    };
    /**```solidity
struct PoolKey { Currency currency0; Currency currency1; uint24 fee; int24 tickSpacing; address hooks; }
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct PoolKey {
        pub currency0: <Currency as alloy::sol_types::SolType>::RustType,
        pub currency1: <Currency as alloy::sol_types::SolType>::RustType,
        pub fee: alloy::sol_types::private::primitives::aliases::U24,
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
        pub hooks: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            Currency,
            Currency,
            alloy::sol_types::sol_data::Uint<24>,
            alloy::sol_types::sol_data::Int<24>,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            <Currency as alloy::sol_types::SolType>::RustType,
            <Currency as alloy::sol_types::SolType>::RustType,
            alloy::sol_types::private::primitives::aliases::U24,
            alloy::sol_types::private::primitives::aliases::I24,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PoolKey> for UnderlyingRustTuple<'_> {
            fn from(value: PoolKey) -> Self {
                (
                    value.currency0,
                    value.currency1,
                    value.fee,
                    value.tickSpacing,
                    value.hooks,
                )
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PoolKey {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currency0: tuple.0,
                    currency1: tuple.1,
                    fee: tuple.2,
                    tickSpacing: tuple.3,
                    hooks: tuple.4,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolValue for PoolKey {
            type SolType = Self;
        }
        #[automatically_derived]
        impl alloy_sol_types::private::SolTypeValue<Self> for PoolKey {
            #[inline]
            fn stv_to_tokens(&self) -> <Self as alloy_sol_types::SolType>::Token<'_> {
                (
                    <Currency as alloy_sol_types::SolType>::tokenize(&self.currency0),
                    <Currency as alloy_sol_types::SolType>::tokenize(&self.currency1),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.hooks,
                    ),
                )
            }
            #[inline]
            fn stv_abi_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encoded_size(&tuple)
            }
            #[inline]
            fn stv_eip712_data_word(&self) -> alloy_sol_types::Word {
                <Self as alloy_sol_types::SolStruct>::eip712_hash_struct(self)
            }
            #[inline]
            fn stv_abi_encode_packed_to(
                &self,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_encode_packed_to(&tuple, out)
            }
            #[inline]
            fn stv_abi_packed_encoded_size(&self) -> usize {
                if let Some(size) = <Self as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE {
                    return size;
                }
                let tuple = <UnderlyingRustTuple<
                    '_,
                > as ::core::convert::From<Self>>::from(self.clone());
                <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_packed_encoded_size(&tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolType for PoolKey {
            type RustType = Self;
            type Token<'a> = <UnderlyingSolTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SOL_NAME: &'static str = <Self as alloy_sol_types::SolStruct>::NAME;
            const ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::ENCODED_SIZE;
            const PACKED_ENCODED_SIZE: Option<usize> = <UnderlyingSolTuple<
                '_,
            > as alloy_sol_types::SolType>::PACKED_ENCODED_SIZE;
            #[inline]
            fn valid_token(token: &Self::Token<'_>) -> bool {
                <UnderlyingSolTuple<'_> as alloy_sol_types::SolType>::valid_token(token)
            }
            #[inline]
            fn detokenize(token: Self::Token<'_>) -> Self::RustType {
                let tuple = <UnderlyingSolTuple<
                    '_,
                > as alloy_sol_types::SolType>::detokenize(token);
                <Self as ::core::convert::From<UnderlyingRustTuple<'_>>>::from(tuple)
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolStruct for PoolKey {
            const NAME: &'static str = "PoolKey";
            #[inline]
            fn eip712_root_type() -> alloy_sol_types::private::Cow<'static, str> {
                alloy_sol_types::private::Cow::Borrowed(
                    "PoolKey(address currency0,address currency1,uint24 fee,int24 tickSpacing,address hooks)",
                )
            }
            #[inline]
            fn eip712_components() -> alloy_sol_types::private::Vec<
                alloy_sol_types::private::Cow<'static, str>,
            > {
                alloy_sol_types::private::Vec::new()
            }
            #[inline]
            fn eip712_encode_type() -> alloy_sol_types::private::Cow<'static, str> {
                <Self as alloy_sol_types::SolStruct>::eip712_root_type()
            }
            #[inline]
            fn eip712_encode_data(&self) -> alloy_sol_types::private::Vec<u8> {
                [
                    <Currency as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currency0,
                        )
                        .0,
                    <Currency as alloy_sol_types::SolType>::eip712_data_word(
                            &self.currency1,
                        )
                        .0,
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.fee)
                        .0,
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::eip712_data_word(&self.tickSpacing)
                        .0,
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::eip712_data_word(
                            &self.hooks,
                        )
                        .0,
                ]
                    .concat()
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::EventTopic for PoolKey {
            #[inline]
            fn topic_preimage_length(rust: &Self::RustType) -> usize {
                0usize
                    + <Currency as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currency0,
                    )
                    + <Currency as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.currency1,
                    )
                    + <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(&rust.fee)
                    + <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.tickSpacing,
                    )
                    + <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::topic_preimage_length(
                        &rust.hooks,
                    )
            }
            #[inline]
            fn encode_topic_preimage(
                rust: &Self::RustType,
                out: &mut alloy_sol_types::private::Vec<u8>,
            ) {
                out.reserve(
                    <Self as alloy_sol_types::EventTopic>::topic_preimage_length(rust),
                );
                <Currency as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.currency0,
                    out,
                );
                <Currency as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.currency1,
                    out,
                );
                <alloy::sol_types::sol_data::Uint<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(&rust.fee, out);
                <alloy::sol_types::sol_data::Int<
                    24,
                > as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.tickSpacing,
                    out,
                );
                <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    &rust.hooks,
                    out,
                );
            }
            #[inline]
            fn encode_topic(
                rust: &Self::RustType,
            ) -> alloy_sol_types::abi::token::WordToken {
                let mut out = alloy_sol_types::private::Vec::new();
                <Self as alloy_sol_types::EventTopic>::encode_topic_preimage(
                    rust,
                    &mut out,
                );
                alloy_sol_types::abi::token::WordToken(
                    alloy_sol_types::private::keccak256(out),
                )
            }
        }
    };
    /**Custom error with signature `AlreadyUnlocked()` and selector `0x5090d6c6`.
```solidity
error AlreadyUnlocked();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct AlreadyUnlocked {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<AlreadyUnlocked> for UnderlyingRustTuple<'_> {
            fn from(value: AlreadyUnlocked) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for AlreadyUnlocked {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for AlreadyUnlocked {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "AlreadyUnlocked()";
            const SELECTOR: [u8; 4] = [80u8, 144u8, 214u8, 198u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ContractUnlocked()` and selector `0x3e5f4fd6`.
```solidity
error ContractUnlocked();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ContractUnlocked {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ContractUnlocked> for UnderlyingRustTuple<'_> {
            fn from(value: ContractUnlocked) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ContractUnlocked {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ContractUnlocked {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ContractUnlocked()";
            const SELECTOR: [u8; 4] = [62u8, 95u8, 79u8, 214u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `CurrenciesOutOfOrderOrEqual(address,address)` and selector `0x6e6c9830`.
```solidity
error CurrenciesOutOfOrderOrEqual(address currency0, address currency1);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct CurrenciesOutOfOrderOrEqual {
        pub currency0: alloy::sol_types::private::Address,
        pub currency1: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (
            alloy::sol_types::sol_data::Address,
            alloy::sol_types::sol_data::Address,
        );
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::Address,
            alloy::sol_types::private::Address,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CurrenciesOutOfOrderOrEqual>
        for UnderlyingRustTuple<'_> {
            fn from(value: CurrenciesOutOfOrderOrEqual) -> Self {
                (value.currency0, value.currency1)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for CurrenciesOutOfOrderOrEqual {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {
                    currency0: tuple.0,
                    currency1: tuple.1,
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CurrenciesOutOfOrderOrEqual {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CurrenciesOutOfOrderOrEqual(address,address)";
            const SELECTOR: [u8; 4] = [110u8, 108u8, 152u8, 48u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.currency0,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.currency1,
                    ),
                )
            }
        }
    };
    /**Custom error with signature `CurrencyNotSettled()` and selector `0x5212cba1`.
```solidity
error CurrencyNotSettled();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct CurrencyNotSettled {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<CurrencyNotSettled> for UnderlyingRustTuple<'_> {
            fn from(value: CurrencyNotSettled) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for CurrencyNotSettled {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for CurrencyNotSettled {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "CurrencyNotSettled()";
            const SELECTOR: [u8; 4] = [82u8, 18u8, 203u8, 161u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `DelegateCallNotAllowed()` and selector `0x0d89438e`.
```solidity
error DelegateCallNotAllowed();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct DelegateCallNotAllowed {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<DelegateCallNotAllowed> for UnderlyingRustTuple<'_> {
            fn from(value: DelegateCallNotAllowed) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for DelegateCallNotAllowed {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for DelegateCallNotAllowed {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "DelegateCallNotAllowed()";
            const SELECTOR: [u8; 4] = [13u8, 137u8, 67u8, 142u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidBips()` and selector `0xdeaa01e6`.
```solidity
error InvalidBips();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidBips {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidBips> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidBips) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidBips {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidBips {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidBips()";
            const SELECTOR: [u8; 4] = [222u8, 170u8, 1u8, 230u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `InvalidCaller()` and selector `0x48f5c3ed`.
```solidity
error InvalidCaller();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct InvalidCaller {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<InvalidCaller> for UnderlyingRustTuple<'_> {
            fn from(value: InvalidCaller) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for InvalidCaller {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for InvalidCaller {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "InvalidCaller()";
            const SELECTOR: [u8; 4] = [72u8, 245u8, 195u8, 237u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ManagerLocked()` and selector `0x54e3ca0d`.
```solidity
error ManagerLocked();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ManagerLocked {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ManagerLocked> for UnderlyingRustTuple<'_> {
            fn from(value: ManagerLocked) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ManagerLocked {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ManagerLocked {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ManagerLocked()";
            const SELECTOR: [u8; 4] = [84u8, 227u8, 202u8, 13u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `MustClearExactPositiveDelta()` and selector `0xbda73abf`.
```solidity
error MustClearExactPositiveDelta();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct MustClearExactPositiveDelta {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<MustClearExactPositiveDelta>
        for UnderlyingRustTuple<'_> {
            fn from(value: MustClearExactPositiveDelta) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for MustClearExactPositiveDelta {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for MustClearExactPositiveDelta {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "MustClearExactPositiveDelta()";
            const SELECTOR: [u8; 4] = [189u8, 167u8, 58u8, 191u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `NonzeroNativeValue()` and selector `0xb0ec849e`.
```solidity
error NonzeroNativeValue();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct NonzeroNativeValue {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<NonzeroNativeValue> for UnderlyingRustTuple<'_> {
            fn from(value: NonzeroNativeValue) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for NonzeroNativeValue {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for NonzeroNativeValue {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "NonzeroNativeValue()";
            const SELECTOR: [u8; 4] = [176u8, 236u8, 132u8, 158u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `PoolNotInitialized()` and selector `0x486aa307`.
```solidity
error PoolNotInitialized();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct PoolNotInitialized {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<PoolNotInitialized> for UnderlyingRustTuple<'_> {
            fn from(value: PoolNotInitialized) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for PoolNotInitialized {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for PoolNotInitialized {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "PoolNotInitialized()";
            const SELECTOR: [u8; 4] = [72u8, 106u8, 163u8, 7u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ProtocolFeeCannotBeFetched()` and selector `0x1ee49702`.
```solidity
error ProtocolFeeCannotBeFetched();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ProtocolFeeCannotBeFetched {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ProtocolFeeCannotBeFetched>
        for UnderlyingRustTuple<'_> {
            fn from(value: ProtocolFeeCannotBeFetched) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for ProtocolFeeCannotBeFetched {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ProtocolFeeCannotBeFetched {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ProtocolFeeCannotBeFetched()";
            const SELECTOR: [u8; 4] = [30u8, 228u8, 151u8, 2u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `ProtocolFeeTooLarge(uint24)` and selector `0xa7abe2f7`.
```solidity
error ProtocolFeeTooLarge(uint24 fee);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ProtocolFeeTooLarge {
        pub fee: alloy::sol_types::private::primitives::aliases::U24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<24>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::U24,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<ProtocolFeeTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: ProtocolFeeTooLarge) -> Self {
                (value.fee,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for ProtocolFeeTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { fee: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for ProtocolFeeTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "ProtocolFeeTooLarge(uint24)";
            const SELECTOR: [u8; 4] = [167u8, 171u8, 226u8, 247u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                )
            }
        }
    };
    /**Custom error with signature `SwapAmountCannotBeZero()` and selector `0xbe8b8507`.
```solidity
error SwapAmountCannotBeZero();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct SwapAmountCannotBeZero {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<SwapAmountCannotBeZero> for UnderlyingRustTuple<'_> {
            fn from(value: SwapAmountCannotBeZero) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for SwapAmountCannotBeZero {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for SwapAmountCannotBeZero {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "SwapAmountCannotBeZero()";
            const SELECTOR: [u8; 4] = [190u8, 139u8, 133u8, 7u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Custom error with signature `TickSpacingTooLarge(int24)` and selector `0xb70024f8`.
```solidity
error TickSpacingTooLarge(int24 tickSpacing);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct TickSpacingTooLarge {
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Int<24>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::I24,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TickSpacingTooLarge> for UnderlyingRustTuple<'_> {
            fn from(value: TickSpacingTooLarge) -> Self {
                (value.tickSpacing,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TickSpacingTooLarge {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { tickSpacing: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TickSpacingTooLarge {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TickSpacingTooLarge(int24)";
            const SELECTOR: [u8; 4] = [183u8, 0u8, 36u8, 248u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                )
            }
        }
    };
    /**Custom error with signature `TickSpacingTooSmall(int24)` and selector `0xe9e90588`.
```solidity
error TickSpacingTooSmall(int24 tickSpacing);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct TickSpacingTooSmall {
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Int<24>,);
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = (
            alloy::sol_types::private::primitives::aliases::I24,
        );
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<TickSpacingTooSmall> for UnderlyingRustTuple<'_> {
            fn from(value: TickSpacingTooSmall) -> Self {
                (value.tickSpacing,)
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>> for TickSpacingTooSmall {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self { tickSpacing: tuple.0 }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for TickSpacingTooSmall {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "TickSpacingTooSmall(int24)";
            const SELECTOR: [u8; 4] = [233u8, 233u8, 5u8, 136u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                )
            }
        }
    };
    /**Custom error with signature `UnauthorizedDynamicLPFeeUpdate()` and selector `0x30d21641`.
```solidity
error UnauthorizedDynamicLPFeeUpdate();
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct UnauthorizedDynamicLPFeeUpdate {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[doc(hidden)]
        type UnderlyingSolTuple<'a> = ();
        #[doc(hidden)]
        type UnderlyingRustTuple<'a> = ();
        #[cfg(test)]
        #[allow(dead_code, unreachable_patterns)]
        fn _type_assertion(
            _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
        ) {
            match _t {
                alloy_sol_types::private::AssertTypeEq::<
                    <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                >(_) => {}
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnauthorizedDynamicLPFeeUpdate>
        for UnderlyingRustTuple<'_> {
            fn from(value: UnauthorizedDynamicLPFeeUpdate) -> Self {
                ()
            }
        }
        #[automatically_derived]
        #[doc(hidden)]
        impl ::core::convert::From<UnderlyingRustTuple<'_>>
        for UnauthorizedDynamicLPFeeUpdate {
            fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                Self {}
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolError for UnauthorizedDynamicLPFeeUpdate {
            type Parameters<'a> = UnderlyingSolTuple<'a>;
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "UnauthorizedDynamicLPFeeUpdate()";
            const SELECTOR: [u8; 4] = [48u8, 210u8, 22u8, 65u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
        }
    };
    /**Event with signature `Approval(address,address,uint256,uint256)` and selector `0xb3fd5071835887567a0671151121894ddccc2842f1d10bedad13e0d17cace9a7`.
```solidity
event Approval(address indexed owner, address indexed spender, uint256 indexed id, uint256 amount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Approval {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub spender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Approval {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "Approval(address,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                179u8,
                253u8,
                80u8,
                113u8,
                131u8,
                88u8,
                135u8,
                86u8,
                122u8,
                6u8,
                113u8,
                21u8,
                17u8,
                33u8,
                137u8,
                77u8,
                220u8,
                204u8,
                40u8,
                66u8,
                241u8,
                209u8,
                11u8,
                237u8,
                173u8,
                19u8,
                224u8,
                209u8,
                124u8,
                172u8,
                233u8,
                167u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    owner: topics.1,
                    spender: topics.2,
                    id: topics.3,
                    amount: data.0,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.owner.clone(),
                    self.spender.clone(),
                    self.id.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.owner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.spender,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.id);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Approval {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Approval> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Approval) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Donate(bytes32,address,uint256,uint256)` and selector `0x29ef05caaff9404b7cb6d1c0e9bbae9eaa7ab2541feba1a9c4248594c08156cb`.
```solidity
event Donate(PoolId indexed id, address indexed sender, uint256 amount0, uint256 amount1);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Donate {
        #[allow(missing_docs)]
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount0: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amount1: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Donate {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                PoolId,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Donate(bytes32,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                41u8,
                239u8,
                5u8,
                202u8,
                175u8,
                249u8,
                64u8,
                75u8,
                124u8,
                182u8,
                209u8,
                192u8,
                233u8,
                187u8,
                174u8,
                158u8,
                170u8,
                122u8,
                178u8,
                84u8,
                31u8,
                235u8,
                161u8,
                169u8,
                196u8,
                36u8,
                133u8,
                148u8,
                192u8,
                129u8,
                86u8,
                203u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    sender: topics.2,
                    amount0: data.0,
                    amount1: data.1,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone(), self.sender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <PoolId as alloy_sol_types::EventTopic>::encode_topic(
                    &self.id,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Donate {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Donate> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Donate) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Initialize(bytes32,address,address,uint24,int24,address,uint160,int24)` and selector `0xdd466e674ea557f56295e2d0218a125ea4b4f0f6f3307b95f85e6110838d6438`.
```solidity
event Initialize(PoolId indexed id, Currency indexed currency0, Currency indexed currency1, uint24 fee, int24 tickSpacing, address hooks, uint160 sqrtPriceX96, int24 tick);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Initialize {
        #[allow(missing_docs)]
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub currency0: <Currency as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub currency1: <Currency as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub fee: alloy::sol_types::private::primitives::aliases::U24,
        #[allow(missing_docs)]
        pub tickSpacing: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub hooks: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub sqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        #[allow(missing_docs)]
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Initialize {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Uint<24>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Int<24>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                PoolId,
                Currency,
                Currency,
            );
            const SIGNATURE: &'static str = "Initialize(bytes32,address,address,uint24,int24,address,uint160,int24)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                221u8,
                70u8,
                110u8,
                103u8,
                78u8,
                165u8,
                87u8,
                245u8,
                98u8,
                149u8,
                226u8,
                208u8,
                33u8,
                138u8,
                18u8,
                94u8,
                164u8,
                180u8,
                240u8,
                246u8,
                243u8,
                48u8,
                123u8,
                149u8,
                248u8,
                94u8,
                97u8,
                16u8,
                131u8,
                141u8,
                100u8,
                56u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    currency0: topics.2,
                    currency1: topics.3,
                    fee: data.0,
                    tickSpacing: data.1,
                    hooks: data.2,
                    sqrtPriceX96: data.3,
                    tick: data.4,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickSpacing),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.hooks,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.sqrtPriceX96),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tick),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.id.clone(),
                    self.currency0.clone(),
                    self.currency1.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <PoolId as alloy_sol_types::EventTopic>::encode_topic(
                    &self.id,
                );
                out[2usize] = <Currency as alloy_sol_types::EventTopic>::encode_topic(
                    &self.currency0,
                );
                out[3usize] = <Currency as alloy_sol_types::EventTopic>::encode_topic(
                    &self.currency1,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Initialize {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Initialize> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Initialize) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ModifyLiquidity(bytes32,address,int24,int24,int256,bytes32)` and selector `0xf208f4912782fd25c7f114ca3723a2d5dd6f3bcc3ac8db5af63baa85f711d5ec`.
```solidity
event ModifyLiquidity(PoolId indexed id, address indexed sender, int24 tickLower, int24 tickUpper, int256 liquidityDelta, bytes32 salt);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct ModifyLiquidity {
        #[allow(missing_docs)]
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub tickLower: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub tickUpper: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub liquidityDelta: alloy::sol_types::private::primitives::aliases::I256,
        #[allow(missing_docs)]
        pub salt: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ModifyLiquidity {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Int<256>,
                alloy::sol_types::sol_data::FixedBytes<32>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                PoolId,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ModifyLiquidity(bytes32,address,int24,int24,int256,bytes32)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                242u8,
                8u8,
                244u8,
                145u8,
                39u8,
                130u8,
                253u8,
                37u8,
                199u8,
                241u8,
                20u8,
                202u8,
                55u8,
                35u8,
                162u8,
                213u8,
                221u8,
                111u8,
                59u8,
                204u8,
                58u8,
                200u8,
                219u8,
                90u8,
                246u8,
                59u8,
                170u8,
                133u8,
                247u8,
                17u8,
                213u8,
                236u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    sender: topics.2,
                    tickLower: data.0,
                    tickUpper: data.1,
                    liquidityDelta: data.2,
                    salt: data.3,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickLower),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tickUpper),
                    <alloy::sol_types::sol_data::Int<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidityDelta),
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.salt),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone(), self.sender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <PoolId as alloy_sol_types::EventTopic>::encode_topic(
                    &self.id,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ModifyLiquidity {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ModifyLiquidity> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ModifyLiquidity) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OperatorSet(address,address,bool)` and selector `0xceb576d9f15e4e200fdb5096d64d5dfd667e16def20c1eefd14256d8e3faa267`.
```solidity
event OperatorSet(address indexed owner, address indexed operator, bool approved);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct OperatorSet {
        #[allow(missing_docs)]
        pub owner: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub operator: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub approved: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OperatorSet {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OperatorSet(address,address,bool)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                206u8,
                181u8,
                118u8,
                217u8,
                241u8,
                94u8,
                78u8,
                32u8,
                15u8,
                219u8,
                80u8,
                150u8,
                214u8,
                77u8,
                93u8,
                253u8,
                102u8,
                126u8,
                22u8,
                222u8,
                242u8,
                12u8,
                30u8,
                239u8,
                209u8,
                66u8,
                86u8,
                216u8,
                227u8,
                250u8,
                162u8,
                103u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    owner: topics.1,
                    operator: topics.2,
                    approved: data.0,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.approved,
                    ),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.owner.clone(), self.operator.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.owner,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.operator,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OperatorSet {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OperatorSet> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OperatorSet) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `OwnershipTransferred(address,address)` and selector `0x8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e0`.
```solidity
event OwnershipTransferred(address indexed user, address indexed newOwner);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct OwnershipTransferred {
        #[allow(missing_docs)]
        pub user: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub newOwner: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for OwnershipTransferred {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "OwnershipTransferred(address,address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    user: topics.1,
                    newOwner: topics.2,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.user.clone(), self.newOwner.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.user,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.newOwner,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for OwnershipTransferred {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&OwnershipTransferred> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &OwnershipTransferred) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ProtocolFeeControllerUpdated(address)` and selector `0xb4bd8ef53df690b9943d3318996006dbb82a25f54719d8c8035b516a2a5b8acc`.
```solidity
event ProtocolFeeControllerUpdated(address indexed protocolFeeController);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct ProtocolFeeControllerUpdated {
        #[allow(missing_docs)]
        pub protocolFeeController: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProtocolFeeControllerUpdated {
            type DataTuple<'a> = ();
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "ProtocolFeeControllerUpdated(address)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                180u8,
                189u8,
                142u8,
                245u8,
                61u8,
                246u8,
                144u8,
                185u8,
                148u8,
                61u8,
                51u8,
                24u8,
                153u8,
                96u8,
                6u8,
                219u8,
                184u8,
                42u8,
                37u8,
                245u8,
                71u8,
                25u8,
                216u8,
                200u8,
                3u8,
                91u8,
                81u8,
                106u8,
                42u8,
                91u8,
                138u8,
                204u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    protocolFeeController: topics.1,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                ()
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.protocolFeeController.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.protocolFeeController,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProtocolFeeControllerUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProtocolFeeControllerUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(
                this: &ProtocolFeeControllerUpdated,
            ) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `ProtocolFeeUpdated(bytes32,uint24)` and selector `0xe9c42593e71f84403b84352cd168d693e2c9fcd1fdbcc3feb21d92b43e6696f9`.
```solidity
event ProtocolFeeUpdated(PoolId indexed id, uint24 protocolFee);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct ProtocolFeeUpdated {
        #[allow(missing_docs)]
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub protocolFee: alloy::sol_types::private::primitives::aliases::U24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for ProtocolFeeUpdated {
            type DataTuple<'a> = (alloy::sol_types::sol_data::Uint<24>,);
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (alloy_sol_types::sol_data::FixedBytes<32>, PoolId);
            const SIGNATURE: &'static str = "ProtocolFeeUpdated(bytes32,uint24)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                233u8,
                196u8,
                37u8,
                147u8,
                231u8,
                31u8,
                132u8,
                64u8,
                59u8,
                132u8,
                53u8,
                44u8,
                209u8,
                104u8,
                214u8,
                147u8,
                226u8,
                201u8,
                252u8,
                209u8,
                253u8,
                188u8,
                195u8,
                254u8,
                178u8,
                29u8,
                146u8,
                180u8,
                62u8,
                102u8,
                150u8,
                249u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    protocolFee: data.0,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.protocolFee),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <PoolId as alloy_sol_types::EventTopic>::encode_topic(
                    &self.id,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for ProtocolFeeUpdated {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&ProtocolFeeUpdated> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &ProtocolFeeUpdated) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Swap(bytes32,address,int128,int128,uint160,uint128,int24,uint24)` and selector `0x40e9cecb9f5f1f1c5b9c97dec2917b7ee92e57ba5563708daca94dd84ad7112f`.
```solidity
event Swap(PoolId indexed id, address indexed sender, int128 amount0, int128 amount1, uint160 sqrtPriceX96, uint128 liquidity, int24 tick, uint24 fee);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Swap {
        #[allow(missing_docs)]
        pub id: <PoolId as alloy::sol_types::SolType>::RustType,
        #[allow(missing_docs)]
        pub sender: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub amount0: i128,
        #[allow(missing_docs)]
        pub amount1: i128,
        #[allow(missing_docs)]
        pub sqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        #[allow(missing_docs)]
        pub liquidity: u128,
        #[allow(missing_docs)]
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
        #[allow(missing_docs)]
        pub fee: alloy::sol_types::private::primitives::aliases::U24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Swap {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Int<128>,
                alloy::sol_types::sol_data::Int<128>,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Uint<128>,
                alloy::sol_types::sol_data::Int<24>,
                alloy::sol_types::sol_data::Uint<24>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                PoolId,
                alloy::sol_types::sol_data::Address,
            );
            const SIGNATURE: &'static str = "Swap(bytes32,address,int128,int128,uint160,uint128,int24,uint24)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                64u8,
                233u8,
                206u8,
                203u8,
                159u8,
                95u8,
                31u8,
                28u8,
                91u8,
                156u8,
                151u8,
                222u8,
                194u8,
                145u8,
                123u8,
                126u8,
                233u8,
                46u8,
                87u8,
                186u8,
                85u8,
                99u8,
                112u8,
                141u8,
                172u8,
                169u8,
                77u8,
                216u8,
                74u8,
                215u8,
                17u8,
                47u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    id: topics.1,
                    sender: topics.2,
                    amount0: data.0,
                    amount1: data.1,
                    sqrtPriceX96: data.2,
                    liquidity: data.3,
                    tick: data.4,
                    fee: data.5,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0),
                    <alloy::sol_types::sol_data::Int<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.sqrtPriceX96),
                    <alloy::sol_types::sol_data::Uint<
                        128,
                    > as alloy_sol_types::SolType>::tokenize(&self.liquidity),
                    <alloy::sol_types::sol_data::Int<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.tick),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.fee),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (Self::SIGNATURE_HASH.into(), self.id.clone(), self.sender.clone())
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <PoolId as alloy_sol_types::EventTopic>::encode_topic(
                    &self.id,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.sender,
                );
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Swap {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Swap> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Swap) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Event with signature `Transfer(address,address,address,uint256,uint256)` and selector `0x1b3d7edb2e9c0b0e7c525b20aaaef0f5940d2ed71663c7d39266ecafac728859`.
```solidity
event Transfer(address caller, address indexed from, address indexed to, uint256 indexed id, uint256 amount);
```*/
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    #[derive(Clone)]
    pub struct Transfer {
        #[allow(missing_docs)]
        pub caller: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub from: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub to: alloy::sol_types::private::Address,
        #[allow(missing_docs)]
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        #[allow(missing_docs)]
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        #[automatically_derived]
        impl alloy_sol_types::SolEvent for Transfer {
            type DataTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type DataToken<'a> = <Self::DataTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type TopicList = (
                alloy_sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            const SIGNATURE: &'static str = "Transfer(address,address,address,uint256,uint256)";
            const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                27u8,
                61u8,
                126u8,
                219u8,
                46u8,
                156u8,
                11u8,
                14u8,
                124u8,
                82u8,
                91u8,
                32u8,
                170u8,
                174u8,
                240u8,
                245u8,
                148u8,
                13u8,
                46u8,
                215u8,
                22u8,
                99u8,
                199u8,
                211u8,
                146u8,
                102u8,
                236u8,
                175u8,
                172u8,
                114u8,
                136u8,
                89u8,
            ]);
            const ANONYMOUS: bool = false;
            #[allow(unused_variables)]
            #[inline]
            fn new(
                topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                Self {
                    caller: data.0,
                    from: topics.1,
                    to: topics.2,
                    id: topics.3,
                    amount: data.1,
                }
            }
            #[inline]
            fn tokenize_body(&self) -> Self::DataToken<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.caller,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn topics(&self) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                (
                    Self::SIGNATURE_HASH.into(),
                    self.from.clone(),
                    self.to.clone(),
                    self.id.clone(),
                )
            }
            #[inline]
            fn encode_topics_raw(
                &self,
                out: &mut [alloy_sol_types::abi::token::WordToken],
            ) -> alloy_sol_types::Result<()> {
                if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT {
                    return Err(alloy_sol_types::Error::Overrun);
                }
                out[0usize] = alloy_sol_types::abi::token::WordToken(
                    Self::SIGNATURE_HASH,
                );
                out[1usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.from,
                );
                out[2usize] = <alloy::sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                    &self.to,
                );
                out[3usize] = <alloy::sol_types::sol_data::Uint<
                    256,
                > as alloy_sol_types::EventTopic>::encode_topic(&self.id);
                Ok(())
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for Transfer {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                From::from(self)
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                From::from(&self)
            }
        }
        #[automatically_derived]
        impl From<&Transfer> for alloy_sol_types::private::LogData {
            #[inline]
            fn from(this: &Transfer) -> alloy_sol_types::private::LogData {
                alloy_sol_types::SolEvent::encode_log_data(this)
            }
        }
    };
    /**Function with signature `allowance(address,address,uint256)` and selector `0x598af9e7`.
```solidity
function allowance(address owner, address spender, uint256 id) external view returns (uint256 amount);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct allowanceCall {
        pub owner: alloy::sol_types::private::Address,
        pub spender: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`allowance(address,address,uint256)`](allowanceCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct allowanceReturn {
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allowanceCall> for UnderlyingRustTuple<'_> {
                fn from(value: allowanceCall) -> Self {
                    (value.owner, value.spender, value.id)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        spender: tuple.1,
                        id: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<allowanceReturn> for UnderlyingRustTuple<'_> {
                fn from(value: allowanceReturn) -> Self {
                    (value.amount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amount: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for allowanceCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = allowanceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "allowance(address,address,uint256)";
            const SELECTOR: [u8; 4] = [89u8, 138u8, 249u8, 231u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `approve(address,uint256,uint256)` and selector `0x426a8493`.
```solidity
function approve(address spender, uint256 id, uint256 amount) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct approveCall {
        pub spender: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`approve(address,uint256,uint256)`](approveCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct approveReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<approveCall> for UnderlyingRustTuple<'_> {
                fn from(value: approveCall) -> Self {
                    (value.spender, value.id, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        spender: tuple.0,
                        id: tuple.1,
                        amount: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<approveReturn> for UnderlyingRustTuple<'_> {
                fn from(value: approveReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for approveCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = approveReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "approve(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [66u8, 106u8, 132u8, 147u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.spender,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `balanceOf(address,uint256)` and selector `0x00fdd58e`.
```solidity
function balanceOf(address owner, uint256 id) external view returns (uint256 balance);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct balanceOfCall {
        pub owner: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`balanceOf(address,uint256)`](balanceOfCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct balanceOfReturn {
        pub balance: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<balanceOfCall> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfCall) -> Self {
                    (value.owner, value.id)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        id: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<balanceOfReturn> for UnderlyingRustTuple<'_> {
                fn from(value: balanceOfReturn) -> Self {
                    (value.balance,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { balance: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for balanceOfCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = balanceOfReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "balanceOf(address,uint256)";
            const SELECTOR: [u8; 4] = [0u8, 253u8, 213u8, 142u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `burn(address,uint256,uint256)` and selector `0xf5298aca`.
```solidity
function burn(address from, uint256 id, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct burnCall {
        pub from: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`burn(address,uint256,uint256)`](burnCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct burnReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<burnCall> for UnderlyingRustTuple<'_> {
                fn from(value: burnCall) -> Self {
                    (value.from, value.id, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        from: tuple.0,
                        id: tuple.1,
                        amount: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<burnReturn> for UnderlyingRustTuple<'_> {
                fn from(value: burnReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for burnReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for burnCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = burnReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "burn(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [245u8, 41u8, 138u8, 202u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.from,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `clear(address,uint256)` and selector `0x80f0b44c`.
```solidity
function clear(Currency currency, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct clearCall {
        pub currency: <Currency as alloy::sol_types::SolType>::RustType,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`clear(address,uint256)`](clearCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct clearReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                Currency,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Currency as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<clearCall> for UnderlyingRustTuple<'_> {
                fn from(value: clearCall) -> Self {
                    (value.currency, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clearCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        currency: tuple.0,
                        amount: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<clearReturn> for UnderlyingRustTuple<'_> {
                fn from(value: clearReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for clearReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for clearCall {
            type Parameters<'a> = (Currency, alloy::sol_types::sol_data::Uint<256>);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = clearReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "clear(address,uint256)";
            const SELECTOR: [u8; 4] = [128u8, 240u8, 180u8, 76u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Currency as alloy_sol_types::SolType>::tokenize(&self.currency),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `collectProtocolFees(address,address,uint256)` and selector `0x8161b874`.
```solidity
function collectProtocolFees(address recipient, Currency currency, uint256 amount) external returns (uint256 amountCollected);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct collectProtocolFeesCall {
        pub recipient: alloy::sol_types::private::Address,
        pub currency: <Currency as alloy::sol_types::SolType>::RustType,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`collectProtocolFees(address,address,uint256)`](collectProtocolFeesCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct collectProtocolFeesReturn {
        pub amountCollected: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                Currency,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                <Currency as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<collectProtocolFeesCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: collectProtocolFeesCall) -> Self {
                    (value.recipient, value.currency, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for collectProtocolFeesCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        recipient: tuple.0,
                        currency: tuple.1,
                        amount: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<collectProtocolFeesReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: collectProtocolFeesReturn) -> Self {
                    (value.amountCollected,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for collectProtocolFeesReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amountCollected: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for collectProtocolFeesCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                Currency,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = collectProtocolFeesReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "collectProtocolFees(address,address,uint256)";
            const SELECTOR: [u8; 4] = [129u8, 97u8, 184u8, 116u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                    <Currency as alloy_sol_types::SolType>::tokenize(&self.currency),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `donate((address,address,uint24,int24,address),uint256,uint256,bytes)` and selector `0x234266d7`.
```solidity
function donate(PoolKey memory key, uint256 amount0, uint256 amount1, bytes memory hookData) external returns (BalanceDelta delta);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct donateCall {
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub amount0: alloy::sol_types::private::primitives::aliases::U256,
        pub amount1: alloy::sol_types::private::primitives::aliases::U256,
        pub hookData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`donate((address,address,uint24,int24,address),uint256,uint256,bytes)`](donateCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct donateReturn {
        pub delta: <BalanceDelta as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolKey,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolKey as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<donateCall> for UnderlyingRustTuple<'_> {
                fn from(value: donateCall) -> Self {
                    (value.key, value.amount0, value.amount1, value.hookData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for donateCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        amount0: tuple.1,
                        amount1: tuple.2,
                        hookData: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BalanceDelta,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BalanceDelta as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<donateReturn> for UnderlyingRustTuple<'_> {
                fn from(value: donateReturn) -> Self {
                    (value.delta,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for donateReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { delta: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for donateCall {
            type Parameters<'a> = (
                PoolKey,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = donateReturn;
            type ReturnTuple<'a> = (BalanceDelta,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "donate((address,address,uint24,int24,address),uint256,uint256,bytes)";
            const SELECTOR: [u8; 4] = [35u8, 66u8, 102u8, 215u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount0),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount1),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.hookData,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `extsload(bytes32)` and selector `0x1e2eaeaf`.
```solidity
function extsload(bytes32 slot) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsload_0Call {
        pub slot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`extsload(bytes32)`](extsload_0Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsload_0Return {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<extsload_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: extsload_0Call) -> Self {
                    (value.slot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsload_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slot: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<extsload_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: extsload_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsload_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for extsload_0Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = extsload_0Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "extsload(bytes32)";
            const SELECTOR: [u8; 4] = [30u8, 46u8, 174u8, 175u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `extsload(bytes32,uint256)` and selector `0x35fd631a`.
```solidity
function extsload(bytes32 startSlot, uint256 nSlots) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsload_1Call {
        pub startSlot: alloy::sol_types::private::FixedBytes<32>,
        pub nSlots: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`extsload(bytes32,uint256)`](extsload_1Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsload_1Return {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::FixedBytes<32>,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<extsload_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: extsload_1Call) -> Self {
                    (value.startSlot, value.nSlots)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsload_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        startSlot: tuple.0,
                        nSlots: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<extsload_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: extsload_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsload_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for extsload_1Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::FixedBytes<32>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = extsload_1Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "extsload(bytes32,uint256)";
            const SELECTOR: [u8; 4] = [53u8, 253u8, 99u8, 26u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.startSlot),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.nSlots),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `extsload(bytes32[])` and selector `0xdbd035ff`.
```solidity
function extsload(bytes32[] memory slots) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsload_2Call {
        pub slots: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`extsload(bytes32[])`](extsload_2Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct extsload_2Return {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<extsload_2Call> for UnderlyingRustTuple<'_> {
                fn from(value: extsload_2Call) -> Self {
                    (value.slots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsload_2Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slots: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<extsload_2Return> for UnderlyingRustTuple<'_> {
                fn from(value: extsload_2Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for extsload_2Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for extsload_2Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = extsload_2Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "extsload(bytes32[])";
            const SELECTOR: [u8; 4] = [219u8, 208u8, 53u8, 255u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.slots),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `exttload(bytes32[])` and selector `0x9bf6645f`.
```solidity
function exttload(bytes32[] memory slots) external view returns (bytes32[] memory);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct exttload_0Call {
        pub slots: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    ///Container type for the return parameters of the [`exttload(bytes32[])`](exttload_0Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct exttload_0Return {
        pub _0: alloy::sol_types::private::Vec<
            alloy::sol_types::private::FixedBytes<32>,
        >,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<exttload_0Call> for UnderlyingRustTuple<'_> {
                fn from(value: exttload_0Call) -> Self {
                    (value.slots,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exttload_0Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slots: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Vec<
                    alloy::sol_types::private::FixedBytes<32>,
                >,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<exttload_0Return> for UnderlyingRustTuple<'_> {
                fn from(value: exttload_0Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exttload_0Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for exttload_0Call {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = exttload_0Return;
            type ReturnTuple<'a> = (
                alloy::sol_types::sol_data::Array<
                    alloy::sol_types::sol_data::FixedBytes<32>,
                >,
            );
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "exttload(bytes32[])";
            const SELECTOR: [u8; 4] = [155u8, 246u8, 100u8, 95u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Array<
                        alloy::sol_types::sol_data::FixedBytes<32>,
                    > as alloy_sol_types::SolType>::tokenize(&self.slots),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `exttload(bytes32)` and selector `0xf135baaa`.
```solidity
function exttload(bytes32 slot) external view returns (bytes32);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct exttload_1Call {
        pub slot: alloy::sol_types::private::FixedBytes<32>,
    }
    ///Container type for the return parameters of the [`exttload(bytes32)`](exttload_1Call) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct exttload_1Return {
        pub _0: alloy::sol_types::private::FixedBytes<32>,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<exttload_1Call> for UnderlyingRustTuple<'_> {
                fn from(value: exttload_1Call) -> Self {
                    (value.slot,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exttload_1Call {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { slot: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<32>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<exttload_1Return> for UnderlyingRustTuple<'_> {
                fn from(value: exttload_1Return) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for exttload_1Return {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for exttload_1Call {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = exttload_1Return;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<32>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "exttload(bytes32)";
            const SELECTOR: [u8; 4] = [241u8, 53u8, 186u8, 170u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        32,
                    > as alloy_sol_types::SolType>::tokenize(&self.slot),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `initialize((address,address,uint24,int24,address),uint160,bytes)` and selector `0x695c5bf5`.
```solidity
function initialize(PoolKey memory key, uint160 sqrtPriceX96, bytes memory hookData) external returns (int24 tick);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializeCall {
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub sqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
        pub hookData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`initialize((address,address,uint24,int24,address),uint160,bytes)`](initializeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct initializeReturn {
        pub tick: alloy::sol_types::private::primitives::aliases::I24,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolKey,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolKey as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U160,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeCall> for UnderlyingRustTuple<'_> {
                fn from(value: initializeCall) -> Self {
                    (value.key, value.sqrtPriceX96, value.hookData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        sqrtPriceX96: tuple.1,
                        hookData: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Int<24>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::I24,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<initializeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: initializeReturn) -> Self {
                    (value.tick,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for initializeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { tick: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for initializeCall {
            type Parameters<'a> = (
                PoolKey,
                alloy::sol_types::sol_data::Uint<160>,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = initializeReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Int<24>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "initialize((address,address,uint24,int24,address),uint160,bytes)";
            const SELECTOR: [u8; 4] = [105u8, 92u8, 91u8, 245u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <alloy::sol_types::sol_data::Uint<
                        160,
                    > as alloy_sol_types::SolType>::tokenize(&self.sqrtPriceX96),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.hookData,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `isOperator(address,address)` and selector `0xb6363cf2`.
```solidity
function isOperator(address owner, address operator) external view returns (bool isOperator);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct isOperatorCall {
        pub owner: alloy::sol_types::private::Address,
        pub operator: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`isOperator(address,address)`](isOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct isOperatorReturn {
        pub isOperator: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorCall) -> Self {
                    (value.owner, value.operator)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        owner: tuple.0,
                        operator: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<isOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: isOperatorReturn) -> Self {
                    (value.isOperator,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for isOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { isOperator: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for isOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = isOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "isOperator(address,address)";
            const SELECTOR: [u8; 4] = [182u8, 54u8, 60u8, 242u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.owner,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `mint(address,uint256,uint256)` and selector `0x156e29f6`.
```solidity
function mint(address to, uint256 id, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct mintCall {
        pub to: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`mint(address,uint256,uint256)`](mintCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct mintReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mintCall> for UnderlyingRustTuple<'_> {
                fn from(value: mintCall) -> Self {
                    (value.to, value.id, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        to: tuple.0,
                        id: tuple.1,
                        amount: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<mintReturn> for UnderlyingRustTuple<'_> {
                fn from(value: mintReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for mintReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for mintCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = mintReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "mint(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [21u8, 110u8, 41u8, 246u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `modifyLiquidity((address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)` and selector `0x5a6bcfda`.
```solidity
function modifyLiquidity(PoolKey memory key, IPoolManager.ModifyLiquidityParams memory params, bytes memory hookData) external returns (BalanceDelta callerDelta, BalanceDelta feesAccrued);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct modifyLiquidityCall {
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
        pub hookData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`modifyLiquidity((address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)`](modifyLiquidityCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct modifyLiquidityReturn {
        pub callerDelta: <BalanceDelta as alloy::sol_types::SolType>::RustType,
        pub feesAccrued: <BalanceDelta as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolKey as alloy::sol_types::SolType>::RustType,
                <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<modifyLiquidityCall> for UnderlyingRustTuple<'_> {
                fn from(value: modifyLiquidityCall) -> Self {
                    (value.key, value.params, value.hookData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for modifyLiquidityCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        params: tuple.1,
                        hookData: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BalanceDelta, BalanceDelta);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BalanceDelta as alloy::sol_types::SolType>::RustType,
                <BalanceDelta as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<modifyLiquidityReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: modifyLiquidityReturn) -> Self {
                    (value.callerDelta, value.feesAccrued)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for modifyLiquidityReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        callerDelta: tuple.0,
                        feesAccrued: tuple.1,
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for modifyLiquidityCall {
            type Parameters<'a> = (
                PoolKey,
                IPoolManager::ModifyLiquidityParams,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = modifyLiquidityReturn;
            type ReturnTuple<'a> = (BalanceDelta, BalanceDelta);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "modifyLiquidity((address,address,uint24,int24,address),(int24,int24,int256,bytes32),bytes)";
            const SELECTOR: [u8; 4] = [90u8, 107u8, 207u8, 218u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <IPoolManager::ModifyLiquidityParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.hookData,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `owner()` and selector `0x8da5cb5b`.
```solidity
function owner() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ownerCall {}
    ///Container type for the return parameters of the [`owner()`](ownerCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct ownerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerCall> for UnderlyingRustTuple<'_> {
                fn from(value: ownerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<ownerReturn> for UnderlyingRustTuple<'_> {
                fn from(value: ownerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for ownerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for ownerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = ownerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "owner()";
            const SELECTOR: [u8; 4] = [141u8, 165u8, 203u8, 91u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `protocolFeeController()` and selector `0xf02de3b2`.
```solidity
function protocolFeeController() external view returns (address);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct protocolFeeControllerCall {}
    ///Container type for the return parameters of the [`protocolFeeController()`](protocolFeeControllerCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct protocolFeeControllerReturn {
        pub _0: alloy::sol_types::private::Address,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<protocolFeeControllerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: protocolFeeControllerCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for protocolFeeControllerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<protocolFeeControllerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: protocolFeeControllerReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for protocolFeeControllerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for protocolFeeControllerCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = protocolFeeControllerReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Address,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "protocolFeeController()";
            const SELECTOR: [u8; 4] = [240u8, 45u8, 227u8, 178u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `protocolFeesAccrued(address)` and selector `0x97e8cd4e`.
```solidity
function protocolFeesAccrued(Currency currency) external view returns (uint256 amount);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct protocolFeesAccruedCall {
        pub currency: <Currency as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`protocolFeesAccrued(address)`](protocolFeesAccruedCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct protocolFeesAccruedReturn {
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Currency,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Currency as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<protocolFeesAccruedCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: protocolFeesAccruedCall) -> Self {
                    (value.currency,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for protocolFeesAccruedCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { currency: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<protocolFeesAccruedReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: protocolFeesAccruedReturn) -> Self {
                    (value.amount,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for protocolFeesAccruedReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { amount: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for protocolFeesAccruedCall {
            type Parameters<'a> = (Currency,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = protocolFeesAccruedReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "protocolFeesAccrued(address)";
            const SELECTOR: [u8; 4] = [151u8, 232u8, 205u8, 78u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Currency as alloy_sol_types::SolType>::tokenize(&self.currency),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setOperator(address,bool)` and selector `0x558a7297`.
```solidity
function setOperator(address operator, bool approved) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setOperatorCall {
        pub operator: alloy::sol_types::private::Address,
        pub approved: bool,
    }
    ///Container type for the return parameters of the [`setOperator(address,bool)`](setOperatorCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setOperatorReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address, bool);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorCall> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorCall) -> Self {
                    (value.operator, value.approved)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        operator: tuple.0,
                        approved: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setOperatorReturn> for UnderlyingRustTuple<'_> {
                fn from(value: setOperatorReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setOperatorReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setOperatorCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Bool,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setOperatorReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setOperator(address,bool)";
            const SELECTOR: [u8; 4] = [85u8, 138u8, 114u8, 151u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.operator,
                    ),
                    <alloy::sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                        &self.approved,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setProtocolFee((address,address,uint24,int24,address),uint24)` and selector `0x7e87ce7d`.
```solidity
function setProtocolFee(PoolKey memory key, uint24 newProtocolFee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setProtocolFeeCall {
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub newProtocolFee: alloy::sol_types::private::primitives::aliases::U24,
    }
    ///Container type for the return parameters of the [`setProtocolFee((address,address,uint24,int24,address),uint24)`](setProtocolFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setProtocolFeeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolKey,
                alloy::sol_types::sol_data::Uint<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolKey as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U24,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setProtocolFeeCall> for UnderlyingRustTuple<'_> {
                fn from(value: setProtocolFeeCall) -> Self {
                    (value.key, value.newProtocolFee)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for setProtocolFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        newProtocolFee: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setProtocolFeeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProtocolFeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProtocolFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setProtocolFeeCall {
            type Parameters<'a> = (PoolKey, alloy::sol_types::sol_data::Uint<24>);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setProtocolFeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setProtocolFee((address,address,uint24,int24,address),uint24)";
            const SELECTOR: [u8; 4] = [126u8, 135u8, 206u8, 125u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.newProtocolFee),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `setProtocolFeeController(address)` and selector `0x2d771389`.
```solidity
function setProtocolFeeController(address controller) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setProtocolFeeControllerCall {
        pub controller: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`setProtocolFeeController(address)`](setProtocolFeeControllerCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct setProtocolFeeControllerReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setProtocolFeeControllerCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProtocolFeeControllerCall) -> Self {
                    (value.controller,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProtocolFeeControllerCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { controller: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<setProtocolFeeControllerReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: setProtocolFeeControllerReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for setProtocolFeeControllerReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for setProtocolFeeControllerCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = setProtocolFeeControllerReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "setProtocolFeeController(address)";
            const SELECTOR: [u8; 4] = [45u8, 119u8, 19u8, 137u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.controller,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `settle()` and selector `0x11da60b4`.
```solidity
function settle() external payable returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct settleCall {}
    ///Container type for the return parameters of the [`settle()`](settleCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct settleReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<settleCall> for UnderlyingRustTuple<'_> {
                fn from(value: settleCall) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for settleCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<settleReturn> for UnderlyingRustTuple<'_> {
                fn from(value: settleReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for settleReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for settleCall {
            type Parameters<'a> = ();
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = settleReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "settle()";
            const SELECTOR: [u8; 4] = [17u8, 218u8, 96u8, 180u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                ()
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `settleFor(address)` and selector `0x3dd45adb`.
```solidity
function settleFor(address recipient) external payable returns (uint256);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct settleForCall {
        pub recipient: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`settleFor(address)`](settleForCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct settleForReturn {
        pub _0: alloy::sol_types::private::primitives::aliases::U256,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<settleForCall> for UnderlyingRustTuple<'_> {
                fn from(value: settleForCall) -> Self {
                    (value.recipient,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for settleForCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { recipient: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<settleForReturn> for UnderlyingRustTuple<'_> {
                fn from(value: settleForReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for settleForReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for settleForCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = settleForReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Uint<256>,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "settleFor(address)";
            const SELECTOR: [u8; 4] = [61u8, 212u8, 90u8, 219u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.recipient,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`.
```solidity
function supportsInterface(bytes4 interfaceId) external view returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct supportsInterfaceCall {
        pub interfaceId: alloy::sol_types::private::FixedBytes<4>,
    }
    ///Container type for the return parameters of the [`supportsInterface(bytes4)`](supportsInterfaceCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct supportsInterfaceReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::FixedBytes<4>,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceCall) -> Self {
                    (value.interfaceId,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { interfaceId: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<supportsInterfaceReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: supportsInterfaceReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for supportsInterfaceReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for supportsInterfaceCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::FixedBytes<4>,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = supportsInterfaceReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "supportsInterface(bytes4)";
            const SELECTOR: [u8; 4] = [1u8, 255u8, 201u8, 167u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::FixedBytes<
                        4,
                    > as alloy_sol_types::SolType>::tokenize(&self.interfaceId),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `swap((address,address,uint24,int24,address),(bool,int256,uint160),bytes)` and selector `0xf3cd914c`.
```solidity
function swap(PoolKey memory key, IPoolManager.SwapParams memory params, bytes memory hookData) external returns (BalanceDelta swapDelta);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct swapCall {
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub params: <IPoolManager::SwapParams as alloy::sol_types::SolType>::RustType,
        pub hookData: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`swap((address,address,uint24,int24,address),(bool,int256,uint160),bytes)`](swapCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct swapReturn {
        pub swapDelta: <BalanceDelta as alloy::sol_types::SolType>::RustType,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolKey,
                IPoolManager::SwapParams,
                alloy::sol_types::sol_data::Bytes,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolKey as alloy::sol_types::SolType>::RustType,
                <IPoolManager::SwapParams as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Bytes,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<swapCall> for UnderlyingRustTuple<'_> {
                fn from(value: swapCall) -> Self {
                    (value.key, value.params, value.hookData)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for swapCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        params: tuple.1,
                        hookData: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (BalanceDelta,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <BalanceDelta as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<swapReturn> for UnderlyingRustTuple<'_> {
                fn from(value: swapReturn) -> Self {
                    (value.swapDelta,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for swapReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { swapDelta: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for swapCall {
            type Parameters<'a> = (
                PoolKey,
                IPoolManager::SwapParams,
                alloy::sol_types::sol_data::Bytes,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = swapReturn;
            type ReturnTuple<'a> = (BalanceDelta,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "swap((address,address,uint24,int24,address),(bool,int256,uint160),bytes)";
            const SELECTOR: [u8; 4] = [243u8, 205u8, 145u8, 76u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <IPoolManager::SwapParams as alloy_sol_types::SolType>::tokenize(
                        &self.params,
                    ),
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.hookData,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `sync(address)` and selector `0xa5841194`.
```solidity
function sync(Currency currency) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct syncCall {
        pub currency: <Currency as alloy::sol_types::SolType>::RustType,
    }
    ///Container type for the return parameters of the [`sync(address)`](syncCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct syncReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (Currency,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Currency as alloy::sol_types::SolType>::RustType,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<syncCall> for UnderlyingRustTuple<'_> {
                fn from(value: syncCall) -> Self {
                    (value.currency,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for syncCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { currency: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<syncReturn> for UnderlyingRustTuple<'_> {
                fn from(value: syncReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for syncReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for syncCall {
            type Parameters<'a> = (Currency,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = syncReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "sync(address)";
            const SELECTOR: [u8; 4] = [165u8, 132u8, 17u8, 148u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (<Currency as alloy_sol_types::SolType>::tokenize(&self.currency),)
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `take(address,address,uint256)` and selector `0x0b0d9c09`.
```solidity
function take(Currency currency, address to, uint256 amount) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct takeCall {
        pub currency: <Currency as alloy::sol_types::SolType>::RustType,
        pub to: alloy::sol_types::private::Address,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`take(address,address,uint256)`](takeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct takeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                Currency,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <Currency as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<takeCall> for UnderlyingRustTuple<'_> {
                fn from(value: takeCall) -> Self {
                    (value.currency, value.to, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for takeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        currency: tuple.0,
                        to: tuple.1,
                        amount: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<takeReturn> for UnderlyingRustTuple<'_> {
                fn from(value: takeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for takeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for takeCall {
            type Parameters<'a> = (
                Currency,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = takeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "take(address,address,uint256)";
            const SELECTOR: [u8; 4] = [11u8, 13u8, 156u8, 9u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <Currency as alloy_sol_types::SolType>::tokenize(&self.currency),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.to,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transfer(address,uint256,uint256)` and selector `0x095bcdb6`.
```solidity
function transfer(address receiver, uint256 id, uint256 amount) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferCall {
        pub receiver: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`transfer(address,uint256,uint256)`](transferCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferCall) -> Self {
                    (value.receiver, value.id, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        receiver: tuple.0,
                        id: tuple.1,
                        amount: tuple.2,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transfer(address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [9u8, 91u8, 205u8, 182u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.receiver,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transferFrom(address,address,uint256,uint256)` and selector `0xfe99049a`.
```solidity
function transferFrom(address sender, address receiver, uint256 id, uint256 amount) external returns (bool);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferFromCall {
        pub sender: alloy::sol_types::private::Address,
        pub receiver: alloy::sol_types::private::Address,
        pub id: alloy::sol_types::private::primitives::aliases::U256,
        pub amount: alloy::sol_types::private::primitives::aliases::U256,
    }
    ///Container type for the return parameters of the [`transferFrom(address,address,uint256,uint256)`](transferFromCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferFromReturn {
        pub _0: bool,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                alloy::sol_types::private::Address,
                alloy::sol_types::private::Address,
                alloy::sol_types::private::primitives::aliases::U256,
                alloy::sol_types::private::primitives::aliases::U256,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferFromCall> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromCall) -> Self {
                    (value.sender, value.receiver, value.id, value.amount)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        sender: tuple.0,
                        receiver: tuple.1,
                        id: tuple.2,
                        amount: tuple.3,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (bool,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferFromReturn> for UnderlyingRustTuple<'_> {
                fn from(value: transferFromReturn) -> Self {
                    (value._0,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferFromReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { _0: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferFromCall {
            type Parameters<'a> = (
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Address,
                alloy::sol_types::sol_data::Uint<256>,
                alloy::sol_types::sol_data::Uint<256>,
            );
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferFromReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bool,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferFrom(address,address,uint256,uint256)";
            const SELECTOR: [u8; 4] = [254u8, 153u8, 4u8, 154u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.sender,
                    ),
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.receiver,
                    ),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.id),
                    <alloy::sol_types::sol_data::Uint<
                        256,
                    > as alloy_sol_types::SolType>::tokenize(&self.amount),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `transferOwnership(address)` and selector `0xf2fde38b`.
```solidity
function transferOwnership(address newOwner) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferOwnershipCall {
        pub newOwner: alloy::sol_types::private::Address,
    }
    ///Container type for the return parameters of the [`transferOwnership(address)`](transferOwnershipCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct transferOwnershipReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Address,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Address,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipCall) -> Self {
                    (value.newOwner,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { newOwner: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<transferOwnershipReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: transferOwnershipReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for transferOwnershipReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for transferOwnershipCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Address,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = transferOwnershipReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "transferOwnership(address)";
            const SELECTOR: [u8; 4] = [242u8, 253u8, 227u8, 139u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                        &self.newOwner,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `unlock(bytes)` and selector `0x48c89491`.
```solidity
function unlock(bytes memory data) external returns (bytes memory result);
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct unlockCall {
        pub data: alloy::sol_types::private::Bytes,
    }
    ///Container type for the return parameters of the [`unlock(bytes)`](unlockCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct unlockReturn {
        pub result: alloy::sol_types::private::Bytes,
    }
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unlockCall> for UnderlyingRustTuple<'_> {
                fn from(value: unlockCall) -> Self {
                    (value.data,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unlockCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { data: tuple.0 }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (alloy::sol_types::private::Bytes,);
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<unlockReturn> for UnderlyingRustTuple<'_> {
                fn from(value: unlockReturn) -> Self {
                    (value.result,)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>> for unlockReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self { result: tuple.0 }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for unlockCall {
            type Parameters<'a> = (alloy::sol_types::sol_data::Bytes,);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = unlockReturn;
            type ReturnTuple<'a> = (alloy::sol_types::sol_data::Bytes,);
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "unlock(bytes)";
            const SELECTOR: [u8; 4] = [72u8, 200u8, 148u8, 145u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <alloy::sol_types::sol_data::Bytes as alloy_sol_types::SolType>::tokenize(
                        &self.data,
                    ),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    /**Function with signature `updateDynamicLPFee((address,address,uint24,int24,address),uint24)` and selector `0x52759651`.
```solidity
function updateDynamicLPFee(PoolKey memory key, uint24 newDynamicLPFee) external;
```*/
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateDynamicLPFeeCall {
        pub key: <PoolKey as alloy::sol_types::SolType>::RustType,
        pub newDynamicLPFee: alloy::sol_types::private::primitives::aliases::U24,
    }
    ///Container type for the return parameters of the [`updateDynamicLPFee((address,address,uint24,int24,address),uint24)`](updateDynamicLPFeeCall) function.
    #[allow(non_camel_case_types, non_snake_case)]
    #[derive(Clone)]
    pub struct updateDynamicLPFeeReturn {}
    #[allow(non_camel_case_types, non_snake_case, clippy::style)]
    const _: () = {
        use alloy::sol_types as alloy_sol_types;
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = (
                PoolKey,
                alloy::sol_types::sol_data::Uint<24>,
            );
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = (
                <PoolKey as alloy::sol_types::SolType>::RustType,
                alloy::sol_types::private::primitives::aliases::U24,
            );
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateDynamicLPFeeCall>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateDynamicLPFeeCall) -> Self {
                    (value.key, value.newDynamicLPFee)
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateDynamicLPFeeCall {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {
                        key: tuple.0,
                        newDynamicLPFee: tuple.1,
                    }
                }
            }
        }
        {
            #[doc(hidden)]
            type UnderlyingSolTuple<'a> = ();
            #[doc(hidden)]
            type UnderlyingRustTuple<'a> = ();
            #[cfg(test)]
            #[allow(dead_code, unreachable_patterns)]
            fn _type_assertion(
                _t: alloy_sol_types::private::AssertTypeEq<UnderlyingRustTuple>,
            ) {
                match _t {
                    alloy_sol_types::private::AssertTypeEq::<
                        <UnderlyingSolTuple as alloy_sol_types::SolType>::RustType,
                    >(_) => {}
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<updateDynamicLPFeeReturn>
            for UnderlyingRustTuple<'_> {
                fn from(value: updateDynamicLPFeeReturn) -> Self {
                    ()
                }
            }
            #[automatically_derived]
            #[doc(hidden)]
            impl ::core::convert::From<UnderlyingRustTuple<'_>>
            for updateDynamicLPFeeReturn {
                fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                    Self {}
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::SolCall for updateDynamicLPFeeCall {
            type Parameters<'a> = (PoolKey, alloy::sol_types::sol_data::Uint<24>);
            type Token<'a> = <Self::Parameters<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            type Return = updateDynamicLPFeeReturn;
            type ReturnTuple<'a> = ();
            type ReturnToken<'a> = <Self::ReturnTuple<
                'a,
            > as alloy_sol_types::SolType>::Token<'a>;
            const SIGNATURE: &'static str = "updateDynamicLPFee((address,address,uint24,int24,address),uint24)";
            const SELECTOR: [u8; 4] = [82u8, 117u8, 150u8, 81u8];
            #[inline]
            fn new<'a>(
                tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
            ) -> Self {
                tuple.into()
            }
            #[inline]
            fn tokenize(&self) -> Self::Token<'_> {
                (
                    <PoolKey as alloy_sol_types::SolType>::tokenize(&self.key),
                    <alloy::sol_types::sol_data::Uint<
                        24,
                    > as alloy_sol_types::SolType>::tokenize(&self.newDynamicLPFee),
                )
            }
            #[inline]
            fn abi_decode_returns(
                data: &[u8],
                validate: bool,
            ) -> alloy_sol_types::Result<Self::Return> {
                <Self::ReturnTuple<
                    '_,
                > as alloy_sol_types::SolType>::abi_decode_sequence(data, validate)
                    .map(Into::into)
            }
        }
    };
    ///Container for all the [`PoolManager`](self) function calls.
    pub enum PoolManagerCalls {
        allowance(allowanceCall),
        approve(approveCall),
        balanceOf(balanceOfCall),
        burn(burnCall),
        clear(clearCall),
        collectProtocolFees(collectProtocolFeesCall),
        donate(donateCall),
        extsload_0(extsload_0Call),
        extsload_1(extsload_1Call),
        extsload_2(extsload_2Call),
        exttload_0(exttload_0Call),
        exttload_1(exttload_1Call),
        initialize(initializeCall),
        isOperator(isOperatorCall),
        mint(mintCall),
        modifyLiquidity(modifyLiquidityCall),
        owner(ownerCall),
        protocolFeeController(protocolFeeControllerCall),
        protocolFeesAccrued(protocolFeesAccruedCall),
        setOperator(setOperatorCall),
        setProtocolFee(setProtocolFeeCall),
        setProtocolFeeController(setProtocolFeeControllerCall),
        settle(settleCall),
        settleFor(settleForCall),
        supportsInterface(supportsInterfaceCall),
        swap(swapCall),
        sync(syncCall),
        take(takeCall),
        transfer(transferCall),
        transferFrom(transferFromCall),
        transferOwnership(transferOwnershipCall),
        unlock(unlockCall),
        updateDynamicLPFee(updateDynamicLPFeeCall),
    }
    #[automatically_derived]
    impl PoolManagerCalls {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [0u8, 253u8, 213u8, 142u8],
            [1u8, 255u8, 201u8, 167u8],
            [9u8, 91u8, 205u8, 182u8],
            [11u8, 13u8, 156u8, 9u8],
            [17u8, 218u8, 96u8, 180u8],
            [21u8, 110u8, 41u8, 246u8],
            [30u8, 46u8, 174u8, 175u8],
            [35u8, 66u8, 102u8, 215u8],
            [45u8, 119u8, 19u8, 137u8],
            [53u8, 253u8, 99u8, 26u8],
            [61u8, 212u8, 90u8, 219u8],
            [66u8, 106u8, 132u8, 147u8],
            [72u8, 200u8, 148u8, 145u8],
            [82u8, 117u8, 150u8, 81u8],
            [85u8, 138u8, 114u8, 151u8],
            [89u8, 138u8, 249u8, 231u8],
            [90u8, 107u8, 207u8, 218u8],
            [105u8, 92u8, 91u8, 245u8],
            [126u8, 135u8, 206u8, 125u8],
            [128u8, 240u8, 180u8, 76u8],
            [129u8, 97u8, 184u8, 116u8],
            [141u8, 165u8, 203u8, 91u8],
            [151u8, 232u8, 205u8, 78u8],
            [155u8, 246u8, 100u8, 95u8],
            [165u8, 132u8, 17u8, 148u8],
            [182u8, 54u8, 60u8, 242u8],
            [219u8, 208u8, 53u8, 255u8],
            [240u8, 45u8, 227u8, 178u8],
            [241u8, 53u8, 186u8, 170u8],
            [242u8, 253u8, 227u8, 139u8],
            [243u8, 205u8, 145u8, 76u8],
            [245u8, 41u8, 138u8, 202u8],
            [254u8, 153u8, 4u8, 154u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for PoolManagerCalls {
        const NAME: &'static str = "PoolManagerCalls";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 33usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::allowance(_) => {
                    <allowanceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::approve(_) => <approveCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::balanceOf(_) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::burn(_) => <burnCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::clear(_) => <clearCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::collectProtocolFees(_) => {
                    <collectProtocolFeesCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::donate(_) => <donateCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::extsload_0(_) => {
                    <extsload_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::extsload_1(_) => {
                    <extsload_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::extsload_2(_) => {
                    <extsload_2Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::exttload_0(_) => {
                    <exttload_0Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::exttload_1(_) => {
                    <exttload_1Call as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::initialize(_) => {
                    <initializeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::isOperator(_) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::mint(_) => <mintCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::modifyLiquidity(_) => {
                    <modifyLiquidityCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::owner(_) => <ownerCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::protocolFeeController(_) => {
                    <protocolFeeControllerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::protocolFeesAccrued(_) => {
                    <protocolFeesAccruedCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setOperator(_) => {
                    <setOperatorCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setProtocolFee(_) => {
                    <setProtocolFeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::setProtocolFeeController(_) => {
                    <setProtocolFeeControllerCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::settle(_) => <settleCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::settleFor(_) => {
                    <settleForCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::supportsInterface(_) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::swap(_) => <swapCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::sync(_) => <syncCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::take(_) => <takeCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transfer(_) => <transferCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::transferFrom(_) => {
                    <transferFromCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::transferOwnership(_) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::SELECTOR
                }
                Self::unlock(_) => <unlockCall as alloy_sol_types::SolCall>::SELECTOR,
                Self::updateDynamicLPFee(_) => {
                    <updateDynamicLPFeeCall as alloy_sol_types::SolCall>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<PoolManagerCalls>] = &[
                {
                    fn balanceOf(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::balanceOf)
                    }
                    balanceOf
                },
                {
                    fn supportsInterface(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::supportsInterface)
                    }
                    supportsInterface
                },
                {
                    fn transfer(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <transferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::transfer)
                    }
                    transfer
                },
                {
                    fn take(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <takeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::take)
                    }
                    take
                },
                {
                    fn settle(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <settleCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::settle)
                    }
                    settle
                },
                {
                    fn mint(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <mintCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::mint)
                    }
                    mint
                },
                {
                    fn extsload_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <extsload_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::extsload_0)
                    }
                    extsload_0
                },
                {
                    fn donate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <donateCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::donate)
                    }
                    donate
                },
                {
                    fn setProtocolFeeController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <setProtocolFeeControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::setProtocolFeeController)
                    }
                    setProtocolFeeController
                },
                {
                    fn extsload_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <extsload_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::extsload_1)
                    }
                    extsload_1
                },
                {
                    fn settleFor(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <settleForCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::settleFor)
                    }
                    settleFor
                },
                {
                    fn approve(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <approveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::approve)
                    }
                    approve
                },
                {
                    fn unlock(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <unlockCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::unlock)
                    }
                    unlock
                },
                {
                    fn updateDynamicLPFee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <updateDynamicLPFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::updateDynamicLPFee)
                    }
                    updateDynamicLPFee
                },
                {
                    fn setOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <setOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::setOperator)
                    }
                    setOperator
                },
                {
                    fn allowance(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <allowanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::allowance)
                    }
                    allowance
                },
                {
                    fn modifyLiquidity(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <modifyLiquidityCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::modifyLiquidity)
                    }
                    modifyLiquidity
                },
                {
                    fn initialize(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <initializeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::initialize)
                    }
                    initialize
                },
                {
                    fn setProtocolFee(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <setProtocolFeeCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::setProtocolFee)
                    }
                    setProtocolFee
                },
                {
                    fn clear(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <clearCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::clear)
                    }
                    clear
                },
                {
                    fn collectProtocolFees(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <collectProtocolFeesCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::collectProtocolFees)
                    }
                    collectProtocolFees
                },
                {
                    fn owner(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <ownerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::owner)
                    }
                    owner
                },
                {
                    fn protocolFeesAccrued(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <protocolFeesAccruedCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::protocolFeesAccrued)
                    }
                    protocolFeesAccrued
                },
                {
                    fn exttload_0(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <exttload_0Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::exttload_0)
                    }
                    exttload_0
                },
                {
                    fn sync(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <syncCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::sync)
                    }
                    sync
                },
                {
                    fn isOperator(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <isOperatorCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::isOperator)
                    }
                    isOperator
                },
                {
                    fn extsload_2(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <extsload_2Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::extsload_2)
                    }
                    extsload_2
                },
                {
                    fn protocolFeeController(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <protocolFeeControllerCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::protocolFeeController)
                    }
                    protocolFeeController
                },
                {
                    fn exttload_1(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <exttload_1Call as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::exttload_1)
                    }
                    exttload_1
                },
                {
                    fn transferOwnership(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <transferOwnershipCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::transferOwnership)
                    }
                    transferOwnership
                },
                {
                    fn swap(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <swapCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::swap)
                    }
                    swap
                },
                {
                    fn burn(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <burnCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::burn)
                    }
                    burn
                },
                {
                    fn transferFrom(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerCalls> {
                        <transferFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerCalls::transferFrom)
                    }
                    transferFrom
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::allowance(inner) => {
                    <allowanceCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::burn(inner) => {
                    <burnCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::clear(inner) => {
                    <clearCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::collectProtocolFees(inner) => {
                    <collectProtocolFeesCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::donate(inner) => {
                    <donateCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::extsload_0(inner) => {
                    <extsload_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::extsload_1(inner) => {
                    <extsload_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::extsload_2(inner) => {
                    <extsload_2Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::exttload_0(inner) => {
                    <exttload_0Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::exttload_1(inner) => {
                    <exttload_1Call as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::isOperator(inner) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::mint(inner) => {
                    <mintCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::modifyLiquidity(inner) => {
                    <modifyLiquidityCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::protocolFeeController(inner) => {
                    <protocolFeeControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::protocolFeesAccrued(inner) => {
                    <protocolFeesAccruedCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setOperator(inner) => {
                    <setOperatorCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setProtocolFee(inner) => {
                    <setProtocolFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::setProtocolFeeController(inner) => {
                    <setProtocolFeeControllerCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::settle(inner) => {
                    <settleCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::settleFor(inner) => {
                    <settleForCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::swap(inner) => {
                    <swapCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::sync(inner) => {
                    <syncCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::take(inner) => {
                    <takeCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
                Self::unlock(inner) => {
                    <unlockCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                }
                Self::updateDynamicLPFee(inner) => {
                    <updateDynamicLPFeeCall as alloy_sol_types::SolCall>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::allowance(inner) => {
                    <allowanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::approve(inner) => {
                    <approveCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::balanceOf(inner) => {
                    <balanceOfCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::burn(inner) => {
                    <burnCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::clear(inner) => {
                    <clearCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::collectProtocolFees(inner) => {
                    <collectProtocolFeesCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::donate(inner) => {
                    <donateCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::extsload_0(inner) => {
                    <extsload_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::extsload_1(inner) => {
                    <extsload_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::extsload_2(inner) => {
                    <extsload_2Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::exttload_0(inner) => {
                    <exttload_0Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::exttload_1(inner) => {
                    <exttload_1Call as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::initialize(inner) => {
                    <initializeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::isOperator(inner) => {
                    <isOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::mint(inner) => {
                    <mintCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::modifyLiquidity(inner) => {
                    <modifyLiquidityCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::owner(inner) => {
                    <ownerCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::protocolFeeController(inner) => {
                    <protocolFeeControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::protocolFeesAccrued(inner) => {
                    <protocolFeesAccruedCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setOperator(inner) => {
                    <setOperatorCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setProtocolFee(inner) => {
                    <setProtocolFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::setProtocolFeeController(inner) => {
                    <setProtocolFeeControllerCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::settle(inner) => {
                    <settleCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::settleFor(inner) => {
                    <settleForCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::supportsInterface(inner) => {
                    <supportsInterfaceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::swap(inner) => {
                    <swapCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::sync(inner) => {
                    <syncCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::take(inner) => {
                    <takeCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::transfer(inner) => {
                    <transferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferFrom(inner) => {
                    <transferFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::transferOwnership(inner) => {
                    <transferOwnershipCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::unlock(inner) => {
                    <unlockCall as alloy_sol_types::SolCall>::abi_encode_raw(inner, out)
                }
                Self::updateDynamicLPFee(inner) => {
                    <updateDynamicLPFeeCall as alloy_sol_types::SolCall>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`PoolManager`](self) custom errors.
    pub enum PoolManagerErrors {
        AlreadyUnlocked(AlreadyUnlocked),
        ContractUnlocked(ContractUnlocked),
        CurrenciesOutOfOrderOrEqual(CurrenciesOutOfOrderOrEqual),
        CurrencyNotSettled(CurrencyNotSettled),
        DelegateCallNotAllowed(DelegateCallNotAllowed),
        InvalidBips(InvalidBips),
        InvalidCaller(InvalidCaller),
        ManagerLocked(ManagerLocked),
        MustClearExactPositiveDelta(MustClearExactPositiveDelta),
        NonzeroNativeValue(NonzeroNativeValue),
        PoolNotInitialized(PoolNotInitialized),
        ProtocolFeeCannotBeFetched(ProtocolFeeCannotBeFetched),
        ProtocolFeeTooLarge(ProtocolFeeTooLarge),
        SwapAmountCannotBeZero(SwapAmountCannotBeZero),
        TickSpacingTooLarge(TickSpacingTooLarge),
        TickSpacingTooSmall(TickSpacingTooSmall),
        UnauthorizedDynamicLPFeeUpdate(UnauthorizedDynamicLPFeeUpdate),
    }
    #[automatically_derived]
    impl PoolManagerErrors {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 4usize]] = &[
            [13u8, 137u8, 67u8, 142u8],
            [30u8, 228u8, 151u8, 2u8],
            [48u8, 210u8, 22u8, 65u8],
            [62u8, 95u8, 79u8, 214u8],
            [72u8, 106u8, 163u8, 7u8],
            [72u8, 245u8, 195u8, 237u8],
            [80u8, 144u8, 214u8, 198u8],
            [82u8, 18u8, 203u8, 161u8],
            [84u8, 227u8, 202u8, 13u8],
            [110u8, 108u8, 152u8, 48u8],
            [167u8, 171u8, 226u8, 247u8],
            [176u8, 236u8, 132u8, 158u8],
            [183u8, 0u8, 36u8, 248u8],
            [189u8, 167u8, 58u8, 191u8],
            [190u8, 139u8, 133u8, 7u8],
            [222u8, 170u8, 1u8, 230u8],
            [233u8, 233u8, 5u8, 136u8],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolInterface for PoolManagerErrors {
        const NAME: &'static str = "PoolManagerErrors";
        const MIN_DATA_LENGTH: usize = 0usize;
        const COUNT: usize = 17usize;
        #[inline]
        fn selector(&self) -> [u8; 4] {
            match self {
                Self::AlreadyUnlocked(_) => {
                    <AlreadyUnlocked as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ContractUnlocked(_) => {
                    <ContractUnlocked as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrenciesOutOfOrderOrEqual(_) => {
                    <CurrenciesOutOfOrderOrEqual as alloy_sol_types::SolError>::SELECTOR
                }
                Self::CurrencyNotSettled(_) => {
                    <CurrencyNotSettled as alloy_sol_types::SolError>::SELECTOR
                }
                Self::DelegateCallNotAllowed(_) => {
                    <DelegateCallNotAllowed as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidBips(_) => {
                    <InvalidBips as alloy_sol_types::SolError>::SELECTOR
                }
                Self::InvalidCaller(_) => {
                    <InvalidCaller as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ManagerLocked(_) => {
                    <ManagerLocked as alloy_sol_types::SolError>::SELECTOR
                }
                Self::MustClearExactPositiveDelta(_) => {
                    <MustClearExactPositiveDelta as alloy_sol_types::SolError>::SELECTOR
                }
                Self::NonzeroNativeValue(_) => {
                    <NonzeroNativeValue as alloy_sol_types::SolError>::SELECTOR
                }
                Self::PoolNotInitialized(_) => {
                    <PoolNotInitialized as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ProtocolFeeCannotBeFetched(_) => {
                    <ProtocolFeeCannotBeFetched as alloy_sol_types::SolError>::SELECTOR
                }
                Self::ProtocolFeeTooLarge(_) => {
                    <ProtocolFeeTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::SwapAmountCannotBeZero(_) => {
                    <SwapAmountCannotBeZero as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TickSpacingTooLarge(_) => {
                    <TickSpacingTooLarge as alloy_sol_types::SolError>::SELECTOR
                }
                Self::TickSpacingTooSmall(_) => {
                    <TickSpacingTooSmall as alloy_sol_types::SolError>::SELECTOR
                }
                Self::UnauthorizedDynamicLPFeeUpdate(_) => {
                    <UnauthorizedDynamicLPFeeUpdate as alloy_sol_types::SolError>::SELECTOR
                }
            }
        }
        #[inline]
        fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
            Self::SELECTORS.get(i).copied()
        }
        #[inline]
        fn valid_selector(selector: [u8; 4]) -> bool {
            Self::SELECTORS.binary_search(&selector).is_ok()
        }
        #[inline]
        #[allow(unsafe_code, non_snake_case)]
        fn abi_decode_raw(
            selector: [u8; 4],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            static DECODE_SHIMS: &[fn(
                &[u8],
                bool,
            ) -> alloy_sol_types::Result<PoolManagerErrors>] = &[
                {
                    fn DelegateCallNotAllowed(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <DelegateCallNotAllowed as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::DelegateCallNotAllowed)
                    }
                    DelegateCallNotAllowed
                },
                {
                    fn ProtocolFeeCannotBeFetched(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <ProtocolFeeCannotBeFetched as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::ProtocolFeeCannotBeFetched)
                    }
                    ProtocolFeeCannotBeFetched
                },
                {
                    fn UnauthorizedDynamicLPFeeUpdate(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <UnauthorizedDynamicLPFeeUpdate as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::UnauthorizedDynamicLPFeeUpdate)
                    }
                    UnauthorizedDynamicLPFeeUpdate
                },
                {
                    fn ContractUnlocked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <ContractUnlocked as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::ContractUnlocked)
                    }
                    ContractUnlocked
                },
                {
                    fn PoolNotInitialized(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <PoolNotInitialized as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::PoolNotInitialized)
                    }
                    PoolNotInitialized
                },
                {
                    fn InvalidCaller(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <InvalidCaller as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::InvalidCaller)
                    }
                    InvalidCaller
                },
                {
                    fn AlreadyUnlocked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <AlreadyUnlocked as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::AlreadyUnlocked)
                    }
                    AlreadyUnlocked
                },
                {
                    fn CurrencyNotSettled(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <CurrencyNotSettled as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::CurrencyNotSettled)
                    }
                    CurrencyNotSettled
                },
                {
                    fn ManagerLocked(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <ManagerLocked as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::ManagerLocked)
                    }
                    ManagerLocked
                },
                {
                    fn CurrenciesOutOfOrderOrEqual(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <CurrenciesOutOfOrderOrEqual as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::CurrenciesOutOfOrderOrEqual)
                    }
                    CurrenciesOutOfOrderOrEqual
                },
                {
                    fn ProtocolFeeTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <ProtocolFeeTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::ProtocolFeeTooLarge)
                    }
                    ProtocolFeeTooLarge
                },
                {
                    fn NonzeroNativeValue(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <NonzeroNativeValue as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::NonzeroNativeValue)
                    }
                    NonzeroNativeValue
                },
                {
                    fn TickSpacingTooLarge(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <TickSpacingTooLarge as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::TickSpacingTooLarge)
                    }
                    TickSpacingTooLarge
                },
                {
                    fn MustClearExactPositiveDelta(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <MustClearExactPositiveDelta as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::MustClearExactPositiveDelta)
                    }
                    MustClearExactPositiveDelta
                },
                {
                    fn SwapAmountCannotBeZero(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <SwapAmountCannotBeZero as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::SwapAmountCannotBeZero)
                    }
                    SwapAmountCannotBeZero
                },
                {
                    fn InvalidBips(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <InvalidBips as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::InvalidBips)
                    }
                    InvalidBips
                },
                {
                    fn TickSpacingTooSmall(
                        data: &[u8],
                        validate: bool,
                    ) -> alloy_sol_types::Result<PoolManagerErrors> {
                        <TickSpacingTooSmall as alloy_sol_types::SolError>::abi_decode_raw(
                                data,
                                validate,
                            )
                            .map(PoolManagerErrors::TickSpacingTooSmall)
                    }
                    TickSpacingTooSmall
                },
            ];
            let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                return Err(
                    alloy_sol_types::Error::unknown_selector(
                        <Self as alloy_sol_types::SolInterface>::NAME,
                        selector,
                    ),
                );
            };
            (unsafe { DECODE_SHIMS.get_unchecked(idx) })(data, validate)
        }
        #[inline]
        fn abi_encoded_size(&self) -> usize {
            match self {
                Self::AlreadyUnlocked(inner) => {
                    <AlreadyUnlocked as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ContractUnlocked(inner) => {
                    <ContractUnlocked as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrenciesOutOfOrderOrEqual(inner) => {
                    <CurrenciesOutOfOrderOrEqual as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::CurrencyNotSettled(inner) => {
                    <CurrencyNotSettled as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::DelegateCallNotAllowed(inner) => {
                    <DelegateCallNotAllowed as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::InvalidBips(inner) => {
                    <InvalidBips as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::InvalidCaller(inner) => {
                    <InvalidCaller as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::ManagerLocked(inner) => {
                    <ManagerLocked as alloy_sol_types::SolError>::abi_encoded_size(inner)
                }
                Self::MustClearExactPositiveDelta(inner) => {
                    <MustClearExactPositiveDelta as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::NonzeroNativeValue(inner) => {
                    <NonzeroNativeValue as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::PoolNotInitialized(inner) => {
                    <PoolNotInitialized as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ProtocolFeeCannotBeFetched(inner) => {
                    <ProtocolFeeCannotBeFetched as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::ProtocolFeeTooLarge(inner) => {
                    <ProtocolFeeTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::SwapAmountCannotBeZero(inner) => {
                    <SwapAmountCannotBeZero as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TickSpacingTooLarge(inner) => {
                    <TickSpacingTooLarge as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::TickSpacingTooSmall(inner) => {
                    <TickSpacingTooSmall as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
                Self::UnauthorizedDynamicLPFeeUpdate(inner) => {
                    <UnauthorizedDynamicLPFeeUpdate as alloy_sol_types::SolError>::abi_encoded_size(
                        inner,
                    )
                }
            }
        }
        #[inline]
        fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
            match self {
                Self::AlreadyUnlocked(inner) => {
                    <AlreadyUnlocked as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ContractUnlocked(inner) => {
                    <ContractUnlocked as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CurrenciesOutOfOrderOrEqual(inner) => {
                    <CurrenciesOutOfOrderOrEqual as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::CurrencyNotSettled(inner) => {
                    <CurrencyNotSettled as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::DelegateCallNotAllowed(inner) => {
                    <DelegateCallNotAllowed as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidBips(inner) => {
                    <InvalidBips as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::InvalidCaller(inner) => {
                    <InvalidCaller as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ManagerLocked(inner) => {
                    <ManagerLocked as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::MustClearExactPositiveDelta(inner) => {
                    <MustClearExactPositiveDelta as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::NonzeroNativeValue(inner) => {
                    <NonzeroNativeValue as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::PoolNotInitialized(inner) => {
                    <PoolNotInitialized as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ProtocolFeeCannotBeFetched(inner) => {
                    <ProtocolFeeCannotBeFetched as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::ProtocolFeeTooLarge(inner) => {
                    <ProtocolFeeTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::SwapAmountCannotBeZero(inner) => {
                    <SwapAmountCannotBeZero as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TickSpacingTooLarge(inner) => {
                    <TickSpacingTooLarge as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::TickSpacingTooSmall(inner) => {
                    <TickSpacingTooSmall as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
                Self::UnauthorizedDynamicLPFeeUpdate(inner) => {
                    <UnauthorizedDynamicLPFeeUpdate as alloy_sol_types::SolError>::abi_encode_raw(
                        inner,
                        out,
                    )
                }
            }
        }
    }
    ///Container for all the [`PoolManager`](self) events.
    pub enum PoolManagerEvents {
        Approval(Approval),
        Donate(Donate),
        Initialize(Initialize),
        ModifyLiquidity(ModifyLiquidity),
        OperatorSet(OperatorSet),
        OwnershipTransferred(OwnershipTransferred),
        ProtocolFeeControllerUpdated(ProtocolFeeControllerUpdated),
        ProtocolFeeUpdated(ProtocolFeeUpdated),
        Swap(Swap),
        Transfer(Transfer),
    }
    #[automatically_derived]
    impl PoolManagerEvents {
        /// All the selectors of this enum.
        ///
        /// Note that the selectors might not be in the same order as the variants.
        /// No guarantees are made about the order of the selectors.
        ///
        /// Prefer using `SolInterface` methods instead.
        pub const SELECTORS: &'static [[u8; 32usize]] = &[
            [
                27u8,
                61u8,
                126u8,
                219u8,
                46u8,
                156u8,
                11u8,
                14u8,
                124u8,
                82u8,
                91u8,
                32u8,
                170u8,
                174u8,
                240u8,
                245u8,
                148u8,
                13u8,
                46u8,
                215u8,
                22u8,
                99u8,
                199u8,
                211u8,
                146u8,
                102u8,
                236u8,
                175u8,
                172u8,
                114u8,
                136u8,
                89u8,
            ],
            [
                41u8,
                239u8,
                5u8,
                202u8,
                175u8,
                249u8,
                64u8,
                75u8,
                124u8,
                182u8,
                209u8,
                192u8,
                233u8,
                187u8,
                174u8,
                158u8,
                170u8,
                122u8,
                178u8,
                84u8,
                31u8,
                235u8,
                161u8,
                169u8,
                196u8,
                36u8,
                133u8,
                148u8,
                192u8,
                129u8,
                86u8,
                203u8,
            ],
            [
                64u8,
                233u8,
                206u8,
                203u8,
                159u8,
                95u8,
                31u8,
                28u8,
                91u8,
                156u8,
                151u8,
                222u8,
                194u8,
                145u8,
                123u8,
                126u8,
                233u8,
                46u8,
                87u8,
                186u8,
                85u8,
                99u8,
                112u8,
                141u8,
                172u8,
                169u8,
                77u8,
                216u8,
                74u8,
                215u8,
                17u8,
                47u8,
            ],
            [
                139u8,
                224u8,
                7u8,
                156u8,
                83u8,
                22u8,
                89u8,
                20u8,
                19u8,
                68u8,
                205u8,
                31u8,
                208u8,
                164u8,
                242u8,
                132u8,
                25u8,
                73u8,
                127u8,
                151u8,
                34u8,
                163u8,
                218u8,
                175u8,
                227u8,
                180u8,
                24u8,
                111u8,
                107u8,
                100u8,
                87u8,
                224u8,
            ],
            [
                179u8,
                253u8,
                80u8,
                113u8,
                131u8,
                88u8,
                135u8,
                86u8,
                122u8,
                6u8,
                113u8,
                21u8,
                17u8,
                33u8,
                137u8,
                77u8,
                220u8,
                204u8,
                40u8,
                66u8,
                241u8,
                209u8,
                11u8,
                237u8,
                173u8,
                19u8,
                224u8,
                209u8,
                124u8,
                172u8,
                233u8,
                167u8,
            ],
            [
                180u8,
                189u8,
                142u8,
                245u8,
                61u8,
                246u8,
                144u8,
                185u8,
                148u8,
                61u8,
                51u8,
                24u8,
                153u8,
                96u8,
                6u8,
                219u8,
                184u8,
                42u8,
                37u8,
                245u8,
                71u8,
                25u8,
                216u8,
                200u8,
                3u8,
                91u8,
                81u8,
                106u8,
                42u8,
                91u8,
                138u8,
                204u8,
            ],
            [
                206u8,
                181u8,
                118u8,
                217u8,
                241u8,
                94u8,
                78u8,
                32u8,
                15u8,
                219u8,
                80u8,
                150u8,
                214u8,
                77u8,
                93u8,
                253u8,
                102u8,
                126u8,
                22u8,
                222u8,
                242u8,
                12u8,
                30u8,
                239u8,
                209u8,
                66u8,
                86u8,
                216u8,
                227u8,
                250u8,
                162u8,
                103u8,
            ],
            [
                221u8,
                70u8,
                110u8,
                103u8,
                78u8,
                165u8,
                87u8,
                245u8,
                98u8,
                149u8,
                226u8,
                208u8,
                33u8,
                138u8,
                18u8,
                94u8,
                164u8,
                180u8,
                240u8,
                246u8,
                243u8,
                48u8,
                123u8,
                149u8,
                248u8,
                94u8,
                97u8,
                16u8,
                131u8,
                141u8,
                100u8,
                56u8,
            ],
            [
                233u8,
                196u8,
                37u8,
                147u8,
                231u8,
                31u8,
                132u8,
                64u8,
                59u8,
                132u8,
                53u8,
                44u8,
                209u8,
                104u8,
                214u8,
                147u8,
                226u8,
                201u8,
                252u8,
                209u8,
                253u8,
                188u8,
                195u8,
                254u8,
                178u8,
                29u8,
                146u8,
                180u8,
                62u8,
                102u8,
                150u8,
                249u8,
            ],
            [
                242u8,
                8u8,
                244u8,
                145u8,
                39u8,
                130u8,
                253u8,
                37u8,
                199u8,
                241u8,
                20u8,
                202u8,
                55u8,
                35u8,
                162u8,
                213u8,
                221u8,
                111u8,
                59u8,
                204u8,
                58u8,
                200u8,
                219u8,
                90u8,
                246u8,
                59u8,
                170u8,
                133u8,
                247u8,
                17u8,
                213u8,
                236u8,
            ],
        ];
    }
    #[automatically_derived]
    impl alloy_sol_types::SolEventInterface for PoolManagerEvents {
        const NAME: &'static str = "PoolManagerEvents";
        const COUNT: usize = 10usize;
        fn decode_raw_log(
            topics: &[alloy_sol_types::Word],
            data: &[u8],
            validate: bool,
        ) -> alloy_sol_types::Result<Self> {
            match topics.first().copied() {
                Some(<Approval as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Approval as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Approval)
                }
                Some(<Donate as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Donate as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Donate)
                }
                Some(<Initialize as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Initialize as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Initialize)
                }
                Some(<ModifyLiquidity as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <ModifyLiquidity as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ModifyLiquidity)
                }
                Some(<OperatorSet as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <OperatorSet as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OperatorSet)
                }
                Some(
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <OwnershipTransferred as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::OwnershipTransferred)
                }
                Some(
                    <ProtocolFeeControllerUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ProtocolFeeControllerUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProtocolFeeControllerUpdated)
                }
                Some(
                    <ProtocolFeeUpdated as alloy_sol_types::SolEvent>::SIGNATURE_HASH,
                ) => {
                    <ProtocolFeeUpdated as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::ProtocolFeeUpdated)
                }
                Some(<Swap as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Swap as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Swap)
                }
                Some(<Transfer as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                    <Transfer as alloy_sol_types::SolEvent>::decode_raw_log(
                            topics,
                            data,
                            validate,
                        )
                        .map(Self::Transfer)
                }
                _ => {
                    alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                        name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                        log: alloy_sol_types::private::Box::new(
                            alloy_sol_types::private::LogData::new_unchecked(
                                topics.to_vec(),
                                data.to_vec().into(),
                            ),
                        ),
                    })
                }
            }
        }
    }
    #[automatically_derived]
    impl alloy_sol_types::private::IntoLogData for PoolManagerEvents {
        fn to_log_data(&self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Donate(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Initialize(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ModifyLiquidity(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProtocolFeeControllerUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::ProtocolFeeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Swap(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::to_log_data(inner)
                }
            }
        }
        fn into_log_data(self) -> alloy_sol_types::private::LogData {
            match self {
                Self::Approval(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Donate(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Initialize(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ModifyLiquidity(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OperatorSet(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::OwnershipTransferred(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProtocolFeeControllerUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::ProtocolFeeUpdated(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Swap(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
                Self::Transfer(inner) => {
                    alloy_sol_types::private::IntoLogData::into_log_data(inner)
                }
            }
        }
    }
    use alloy::contract as alloy_contract;
    /**Creates a new wrapper around an on-chain [`PoolManager`](self) contract instance.

See the [wrapper's documentation](`PoolManagerInstance`) for more details.*/
    #[inline]
    pub const fn new<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        address: alloy_sol_types::private::Address,
        provider: P,
    ) -> PoolManagerInstance<T, P, N> {
        PoolManagerInstance::<T, P, N>::new(address, provider)
    }
    /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
    #[inline]
    pub fn deploy<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(
        provider: P,
    ) -> impl ::core::future::Future<
        Output = alloy_contract::Result<PoolManagerInstance<T, P, N>>,
    > {
        PoolManagerInstance::<T, P, N>::deploy(provider)
    }
    /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
    #[inline]
    pub fn deploy_builder<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    >(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
        PoolManagerInstance::<T, P, N>::deploy_builder(provider)
    }
    /**A [`PoolManager`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`PoolManager`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
    #[derive(Clone)]
    pub struct PoolManagerInstance<T, P, N = alloy_contract::private::Ethereum> {
        address: alloy_sol_types::private::Address,
        provider: P,
        _network_transport: ::core::marker::PhantomData<(N, T)>,
    }
    #[automatically_derived]
    impl<T, P, N> ::core::fmt::Debug for PoolManagerInstance<T, P, N> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple("PoolManagerInstance").field(&self.address).finish()
        }
    }
    /// Instantiation and getters/setters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolManagerInstance<T, P, N> {
        /**Creates a new wrapper around an on-chain [`PoolManager`](self) contract instance.

See the [wrapper's documentation](`PoolManagerInstance`) for more details.*/
        #[inline]
        pub const fn new(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> Self {
            Self {
                address,
                provider,
                _network_transport: ::core::marker::PhantomData,
            }
        }
        /**Deploys this contract using the given `provider` and constructor arguments, if any.

Returns a new instance of the contract, if the deployment was successful.

For more fine-grained control over the deployment process, use [`deploy_builder`] instead.*/
        #[inline]
        pub async fn deploy(
            provider: P,
        ) -> alloy_contract::Result<PoolManagerInstance<T, P, N>> {
            let call_builder = Self::deploy_builder(provider);
            let contract_address = call_builder.deploy().await?;
            Ok(Self::new(contract_address, call_builder.provider))
        }
        /**Creates a `RawCallBuilder` for deploying this contract using the given `provider`
and constructor arguments, if any.

This is a simple wrapper around creating a `RawCallBuilder` with the data set to
the bytecode concatenated with the constructor's ABI-encoded arguments.*/
        #[inline]
        pub fn deploy_builder(provider: P) -> alloy_contract::RawCallBuilder<T, P, N> {
            alloy_contract::RawCallBuilder::new_raw_deploy(
                provider,
                ::core::clone::Clone::clone(&BYTECODE),
            )
        }
        /// Returns a reference to the address.
        #[inline]
        pub const fn address(&self) -> &alloy_sol_types::private::Address {
            &self.address
        }
        /// Sets the address.
        #[inline]
        pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
            self.address = address;
        }
        /// Sets the address and returns `self`.
        pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
            self.set_address(address);
            self
        }
        /// Returns a reference to the provider.
        #[inline]
        pub const fn provider(&self) -> &P {
            &self.provider
        }
    }
    impl<T, P: ::core::clone::Clone, N> PoolManagerInstance<T, &P, N> {
        /// Clones the provider and returns a new instance with the cloned provider.
        #[inline]
        pub fn with_cloned_provider(self) -> PoolManagerInstance<T, P, N> {
            PoolManagerInstance {
                address: self.address,
                provider: ::core::clone::Clone::clone(&self.provider),
                _network_transport: ::core::marker::PhantomData,
            }
        }
    }
    /// Function calls.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolManagerInstance<T, P, N> {
        /// Creates a new call builder using this contract instance's provider and address.
        ///
        /// Note that the call can be any function call, not just those defined in this
        /// contract. Prefer using the other methods for building type-safe contract calls.
        pub fn call_builder<C: alloy_sol_types::SolCall>(
            &self,
            call: &C,
        ) -> alloy_contract::SolCallBuilder<T, &P, C, N> {
            alloy_contract::SolCallBuilder::new_sol(&self.provider, &self.address, call)
        }
        ///Creates a new call builder for the [`allowance`] function.
        pub fn allowance(
            &self,
            owner: alloy::sol_types::private::Address,
            spender: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, allowanceCall, N> {
            self.call_builder(
                &allowanceCall {
                    owner,
                    spender,
                    id,
                },
            )
        }
        ///Creates a new call builder for the [`approve`] function.
        pub fn approve(
            &self,
            spender: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, approveCall, N> {
            self.call_builder(&approveCall { spender, id, amount })
        }
        ///Creates a new call builder for the [`balanceOf`] function.
        pub fn balanceOf(
            &self,
            owner: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, balanceOfCall, N> {
            self.call_builder(&balanceOfCall { owner, id })
        }
        ///Creates a new call builder for the [`burn`] function.
        pub fn burn(
            &self,
            from: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, burnCall, N> {
            self.call_builder(&burnCall { from, id, amount })
        }
        ///Creates a new call builder for the [`clear`] function.
        pub fn clear(
            &self,
            currency: <Currency as alloy::sol_types::SolType>::RustType,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, clearCall, N> {
            self.call_builder(&clearCall { currency, amount })
        }
        ///Creates a new call builder for the [`collectProtocolFees`] function.
        pub fn collectProtocolFees(
            &self,
            recipient: alloy::sol_types::private::Address,
            currency: <Currency as alloy::sol_types::SolType>::RustType,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, collectProtocolFeesCall, N> {
            self.call_builder(
                &collectProtocolFeesCall {
                    recipient,
                    currency,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`donate`] function.
        pub fn donate(
            &self,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            amount0: alloy::sol_types::private::primitives::aliases::U256,
            amount1: alloy::sol_types::private::primitives::aliases::U256,
            hookData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, donateCall, N> {
            self.call_builder(
                &donateCall {
                    key,
                    amount0,
                    amount1,
                    hookData,
                },
            )
        }
        ///Creates a new call builder for the [`extsload_0`] function.
        pub fn extsload_0(
            &self,
            slot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, extsload_0Call, N> {
            self.call_builder(&extsload_0Call { slot })
        }
        ///Creates a new call builder for the [`extsload_1`] function.
        pub fn extsload_1(
            &self,
            startSlot: alloy::sol_types::private::FixedBytes<32>,
            nSlots: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, extsload_1Call, N> {
            self.call_builder(
                &extsload_1Call {
                    startSlot,
                    nSlots,
                },
            )
        }
        ///Creates a new call builder for the [`extsload_2`] function.
        pub fn extsload_2(
            &self,
            slots: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, extsload_2Call, N> {
            self.call_builder(&extsload_2Call { slots })
        }
        ///Creates a new call builder for the [`exttload_0`] function.
        pub fn exttload_0(
            &self,
            slots: alloy::sol_types::private::Vec<
                alloy::sol_types::private::FixedBytes<32>,
            >,
        ) -> alloy_contract::SolCallBuilder<T, &P, exttload_0Call, N> {
            self.call_builder(&exttload_0Call { slots })
        }
        ///Creates a new call builder for the [`exttload_1`] function.
        pub fn exttload_1(
            &self,
            slot: alloy::sol_types::private::FixedBytes<32>,
        ) -> alloy_contract::SolCallBuilder<T, &P, exttload_1Call, N> {
            self.call_builder(&exttload_1Call { slot })
        }
        ///Creates a new call builder for the [`initialize`] function.
        pub fn initialize(
            &self,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            sqrtPriceX96: alloy::sol_types::private::primitives::aliases::U160,
            hookData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, initializeCall, N> {
            self.call_builder(
                &initializeCall {
                    key,
                    sqrtPriceX96,
                    hookData,
                },
            )
        }
        ///Creates a new call builder for the [`isOperator`] function.
        pub fn isOperator(
            &self,
            owner: alloy::sol_types::private::Address,
            operator: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, isOperatorCall, N> {
            self.call_builder(&isOperatorCall { owner, operator })
        }
        ///Creates a new call builder for the [`mint`] function.
        pub fn mint(
            &self,
            to: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, mintCall, N> {
            self.call_builder(&mintCall { to, id, amount })
        }
        ///Creates a new call builder for the [`modifyLiquidity`] function.
        pub fn modifyLiquidity(
            &self,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            params: <IPoolManager::ModifyLiquidityParams as alloy::sol_types::SolType>::RustType,
            hookData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, modifyLiquidityCall, N> {
            self.call_builder(
                &modifyLiquidityCall {
                    key,
                    params,
                    hookData,
                },
            )
        }
        ///Creates a new call builder for the [`owner`] function.
        pub fn owner(&self) -> alloy_contract::SolCallBuilder<T, &P, ownerCall, N> {
            self.call_builder(&ownerCall {})
        }
        ///Creates a new call builder for the [`protocolFeeController`] function.
        pub fn protocolFeeController(
            &self,
        ) -> alloy_contract::SolCallBuilder<T, &P, protocolFeeControllerCall, N> {
            self.call_builder(&protocolFeeControllerCall {})
        }
        ///Creates a new call builder for the [`protocolFeesAccrued`] function.
        pub fn protocolFeesAccrued(
            &self,
            currency: <Currency as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, protocolFeesAccruedCall, N> {
            self.call_builder(
                &protocolFeesAccruedCall {
                    currency,
                },
            )
        }
        ///Creates a new call builder for the [`setOperator`] function.
        pub fn setOperator(
            &self,
            operator: alloy::sol_types::private::Address,
            approved: bool,
        ) -> alloy_contract::SolCallBuilder<T, &P, setOperatorCall, N> {
            self.call_builder(
                &setOperatorCall {
                    operator,
                    approved,
                },
            )
        }
        ///Creates a new call builder for the [`setProtocolFee`] function.
        pub fn setProtocolFee(
            &self,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            newProtocolFee: alloy::sol_types::private::primitives::aliases::U24,
        ) -> alloy_contract::SolCallBuilder<T, &P, setProtocolFeeCall, N> {
            self.call_builder(
                &setProtocolFeeCall {
                    key,
                    newProtocolFee,
                },
            )
        }
        ///Creates a new call builder for the [`setProtocolFeeController`] function.
        pub fn setProtocolFeeController(
            &self,
            controller: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, setProtocolFeeControllerCall, N> {
            self.call_builder(
                &setProtocolFeeControllerCall {
                    controller,
                },
            )
        }
        ///Creates a new call builder for the [`settle`] function.
        pub fn settle(&self) -> alloy_contract::SolCallBuilder<T, &P, settleCall, N> {
            self.call_builder(&settleCall {})
        }
        ///Creates a new call builder for the [`settleFor`] function.
        pub fn settleFor(
            &self,
            recipient: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, settleForCall, N> {
            self.call_builder(&settleForCall { recipient })
        }
        ///Creates a new call builder for the [`supportsInterface`] function.
        pub fn supportsInterface(
            &self,
            interfaceId: alloy::sol_types::private::FixedBytes<4>,
        ) -> alloy_contract::SolCallBuilder<T, &P, supportsInterfaceCall, N> {
            self.call_builder(
                &supportsInterfaceCall {
                    interfaceId,
                },
            )
        }
        ///Creates a new call builder for the [`swap`] function.
        pub fn swap(
            &self,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            params: <IPoolManager::SwapParams as alloy::sol_types::SolType>::RustType,
            hookData: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, swapCall, N> {
            self.call_builder(&swapCall { key, params, hookData })
        }
        ///Creates a new call builder for the [`sync`] function.
        pub fn sync(
            &self,
            currency: <Currency as alloy::sol_types::SolType>::RustType,
        ) -> alloy_contract::SolCallBuilder<T, &P, syncCall, N> {
            self.call_builder(&syncCall { currency })
        }
        ///Creates a new call builder for the [`take`] function.
        pub fn take(
            &self,
            currency: <Currency as alloy::sol_types::SolType>::RustType,
            to: alloy::sol_types::private::Address,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, takeCall, N> {
            self.call_builder(&takeCall { currency, to, amount })
        }
        ///Creates a new call builder for the [`transfer`] function.
        pub fn transfer(
            &self,
            receiver: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferCall, N> {
            self.call_builder(
                &transferCall {
                    receiver,
                    id,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`transferFrom`] function.
        pub fn transferFrom(
            &self,
            sender: alloy::sol_types::private::Address,
            receiver: alloy::sol_types::private::Address,
            id: alloy::sol_types::private::primitives::aliases::U256,
            amount: alloy::sol_types::private::primitives::aliases::U256,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferFromCall, N> {
            self.call_builder(
                &transferFromCall {
                    sender,
                    receiver,
                    id,
                    amount,
                },
            )
        }
        ///Creates a new call builder for the [`transferOwnership`] function.
        pub fn transferOwnership(
            &self,
            newOwner: alloy::sol_types::private::Address,
        ) -> alloy_contract::SolCallBuilder<T, &P, transferOwnershipCall, N> {
            self.call_builder(&transferOwnershipCall { newOwner })
        }
        ///Creates a new call builder for the [`unlock`] function.
        pub fn unlock(
            &self,
            data: alloy::sol_types::private::Bytes,
        ) -> alloy_contract::SolCallBuilder<T, &P, unlockCall, N> {
            self.call_builder(&unlockCall { data })
        }
        ///Creates a new call builder for the [`updateDynamicLPFee`] function.
        pub fn updateDynamicLPFee(
            &self,
            key: <PoolKey as alloy::sol_types::SolType>::RustType,
            newDynamicLPFee: alloy::sol_types::private::primitives::aliases::U24,
        ) -> alloy_contract::SolCallBuilder<T, &P, updateDynamicLPFeeCall, N> {
            self.call_builder(
                &updateDynamicLPFeeCall {
                    key,
                    newDynamicLPFee,
                },
            )
        }
    }
    /// Event filters.
    #[automatically_derived]
    impl<
        T: alloy_contract::private::Transport + ::core::clone::Clone,
        P: alloy_contract::private::Provider<T, N>,
        N: alloy_contract::private::Network,
    > PoolManagerInstance<T, P, N> {
        /// Creates a new event filter using this contract instance's provider and address.
        ///
        /// Note that the type can be any event, not just those defined in this contract.
        /// Prefer using the other methods for building type-safe event filters.
        pub fn event_filter<E: alloy_sol_types::SolEvent>(
            &self,
        ) -> alloy_contract::Event<T, &P, E, N> {
            alloy_contract::Event::new_sol(&self.provider, &self.address)
        }
        ///Creates a new event filter for the [`Approval`] event.
        pub fn Approval_filter(&self) -> alloy_contract::Event<T, &P, Approval, N> {
            self.event_filter::<Approval>()
        }
        ///Creates a new event filter for the [`Donate`] event.
        pub fn Donate_filter(&self) -> alloy_contract::Event<T, &P, Donate, N> {
            self.event_filter::<Donate>()
        }
        ///Creates a new event filter for the [`Initialize`] event.
        pub fn Initialize_filter(&self) -> alloy_contract::Event<T, &P, Initialize, N> {
            self.event_filter::<Initialize>()
        }
        ///Creates a new event filter for the [`ModifyLiquidity`] event.
        pub fn ModifyLiquidity_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ModifyLiquidity, N> {
            self.event_filter::<ModifyLiquidity>()
        }
        ///Creates a new event filter for the [`OperatorSet`] event.
        pub fn OperatorSet_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OperatorSet, N> {
            self.event_filter::<OperatorSet>()
        }
        ///Creates a new event filter for the [`OwnershipTransferred`] event.
        pub fn OwnershipTransferred_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, OwnershipTransferred, N> {
            self.event_filter::<OwnershipTransferred>()
        }
        ///Creates a new event filter for the [`ProtocolFeeControllerUpdated`] event.
        pub fn ProtocolFeeControllerUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProtocolFeeControllerUpdated, N> {
            self.event_filter::<ProtocolFeeControllerUpdated>()
        }
        ///Creates a new event filter for the [`ProtocolFeeUpdated`] event.
        pub fn ProtocolFeeUpdated_filter(
            &self,
        ) -> alloy_contract::Event<T, &P, ProtocolFeeUpdated, N> {
            self.event_filter::<ProtocolFeeUpdated>()
        }
        ///Creates a new event filter for the [`Swap`] event.
        pub fn Swap_filter(&self) -> alloy_contract::Event<T, &P, Swap, N> {
            self.event_filter::<Swap>()
        }
        ///Creates a new event filter for the [`Transfer`] event.
        pub fn Transfer_filter(&self) -> alloy_contract::Event<T, &P, Transfer, N> {
            self.event_filter::<Transfer>()
        }
    }
}
