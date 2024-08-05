// SPDX-License-Identifier: BUSL-1.1
pragma solidity ^0.8.13;

import {EXPECTED_HOOK_RETURN_MAGIC} from "../interfaces/IAngstromComposable.sol";
import {OrderVariant} from "./OrderVariant.sol";
import {CalldataReader} from "./CalldataReader.sol";

/// @dev 0 or packed (u64 memory pointer ++ u160 hook address ++ u32 calldata length)
type HookBuffer is uint256;

using HookBufferLib for HookBuffer global;

/// @author philogy <https://github.com/philogy>
library HookBufferLib {
    error InvalidHookReturn();

    /// @dev Hash of empty sequence of bytes `keccak256("")`
    /// @custom:test test/types/OrderIterator.t.sol:test_emptyBytesHash
    uint256 internal constant EMPTY_BYTES_HASH = 0xc5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470;

    uint256 internal constant HOOK_ADDR_OFFSET = 32;
    uint256 internal constant HOOK_MEM_PTR_OFFSET = 192;
    uint256 internal constant HOOK_LENGTH_MASK = 0xffffffff;

    /// @dev Left-shifted hook selector (`compose(address, bytes)`).
    uint256 internal constant HOOK_SELECTOR_LEFT_ALIGNED =
        0x7407905c00000000000000000000000000000000000000000000000000000000;

    function readFrom(CalldataReader reader, OrderVariant variant)
        internal
        pure
        returns (CalldataReader, HookBuffer hook, bytes32 hash)
    {
        bool noHookToRead = variant.noHook();
        assembly ("memory-safe") {
            hook := 0
            hash := EMPTY_BYTES_HASH
            if iszero(noHookToRead) {
                // Load length of address + payload from reader.
                let hookDatalength := shr(232, calldataload(reader))
                reader := add(reader, 3)

                // Allocate memory for hook call.
                let memPtr := mload(0x40)
                let contentOffset := add(memPtr, sub(0x64, 20))
                mstore(0x40, add(contentOffset, hookDatalength))

                // Copy hook data into memory and hash.
                calldatacopy(contentOffset, reader, hookDatalength)
                hash := keccak256(contentOffset, hookDatalength)
                reader := add(reader, hookDatalength)

                // Load hook address from memory..
                let hookAddr := mload(add(memPtr, 0x44))

                // Setup memory for full call.
                mstore(memPtr, HOOK_SELECTOR_LEFT_ALIGNED) // 0x00:0x04 selector
                mstore(add(memPtr, 0x24), 0x40) // 0x24:0x44 calldata offset
                let payloadLength := sub(hookDatalength, 20)
                mstore(add(memPtr, 0x44), payloadLength) // 0x44:0x64 payload length

                // Build packed hook pointer.
                hook :=
                    or(shl(HOOK_MEM_PTR_OFFSET, memPtr), or(shl(HOOK_ADDR_OFFSET, hookAddr), add(payloadLength, 0x64)))
            }
        }

        return (reader, hook, hash);
    }

    /// @dev WARNING: Attempts to free the allocated memory after triggering the hook, use after a
    /// call to this method is *unsafe*.
    function tryTriggerAndFree(HookBuffer self, address from) internal {
        assembly ("memory-safe") {
            if self {
                // Unpack hook.
                let calldataLength := and(self, HOOK_LENGTH_MASK)
                let memPtr := shr(HOOK_MEM_PTR_OFFSET, self)
                // Encode `from`.
                mstore(add(memPtr, 0x04), from)
                // Call hook. The upper bytes of `hookAddr` will be dirty from the memory pointer
                // but the EVM discards upper bytes for calls. https://ethereum.github.io/execution-specs/src/ethereum/cancun/vm/instructions/system.py.html#ethereum.cancun.vm.instructions.system.call:0
                let hookAddr := shr(HOOK_ADDR_OFFSET, self)
                let success := call(gas(), hookAddr, 0, memPtr, calldataLength, 0x00, 0x20)

                // Check that the call was successful, sufficient data was returned and the expected
                // return magic was returned.
                if iszero(and(success, and(gt(returndatasize(), 31), eq(mload(0x00), EXPECTED_HOOK_RETURN_MAGIC)))) {
                    mstore(0x00, 0xf959fdae /* InvalidHookReturn() */ )
                    revert(0x1c, 0x04)
                }

                // - "What allocator? I am the allocator."
                // Checks if end of hook memory allocation is free so we can move down the free
                // pointer, effectively freeing the memory.
                if eq(mload(0x40), add(memPtr, calldataLength)) { mstore(0x40, memPtr) }
            }
        }
    }
}
