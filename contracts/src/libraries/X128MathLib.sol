// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/// @author philogy <https://github.com/philogy>
library X128MathLib {
    error FullMulX128Failed();

    /// @dev Computes `(numerator * 2**128) / denominator` returning `0` if `denominator = 0`
    /// instead of reverting.
    function flatDivX128(uint128 numerator, uint256 denominator)
        internal
        pure
        returns (uint256 result)
    {
        assembly ("memory-safe") {
            result := div(shl(128, numerator), denominator)
        }
    }

    function fullMulX128(uint256 x, uint256 y) internal pure returns (uint256 z) {
        assembly ("memory-safe") {
            z := mul(x, y)
            for {} 1 {} {
                if iszero(or(iszero(x), eq(div(z, x), y))) {
                    let mm := mulmod(x, y, not(0))
                    let p1 := sub(mm, add(z, lt(mm, z))) // Upper 256 bits of `x * y`.

                    // Checks that result isn't overflowing 256 bits.
                    if iszero(lt(p1, shl(128, 1))) {
                        mstore(0x00, 0xc56a0159 /* FullMulX128Failed() */ )
                        revert(0x1c, 0x04)
                    }

                    z := or(shl(128, p1), shr(128, z))
                    break
                }

                z := shr(128, z)
                break
            }
        }
    }
}
