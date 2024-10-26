// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {BaseTest} from "test/_helpers/BaseTest.sol";
import {X128MathLib} from "src/libraries/X128MathLib.sol";
import {FixedPointMathLib} from "solady/src/utils/FixedPointMathLib.sol";

/// @author philogy <https://github.com/philogy>
contract X128MathLibTest is BaseTest {
    function check_matchesSolady_fullMulX128(uint256 x, uint256 y) public view {
        test_fuzzing_matchesSolady_fullMulX128(x, y);
    }

    function test_fuzzing_matchesSolady_fullMulX128(uint256 x, uint256 y) public view {
        bool success;
        uint256 result;

        try this.fullMulX128(x, y) returns (uint256 out) {
            success = true;
            result = out;
        } catch {
            success = false;
        }

        try this.solady_fullMulDiv128(x, y) returns (uint256 out) {
            assertTrue(success, "Solady succeeded when x128 didn't");
            assertEq(out, result, "Different results");
        } catch {
            assertFalse(success, "Solady failed when x128 didn't");
        }
    }

    function fullMulX128(uint256 x, uint256 y) external pure returns (uint256) {
        return X128MathLib.fullMulX128(x, y);
    }

    function solady_fullMulDiv128(uint256 x, uint256 y) external pure returns (uint256) {
        return FixedPointMathLib.fullMulDiv(x, y, 1 << 128);
    }
}
