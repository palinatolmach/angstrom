// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import {BaseTest} from "test/_helpers/BaseTest.sol";
import {Pair as RefPair, PairLib as RefPairLib, PriceAB} from "test/_reference/Pair.sol";
import {CalldataReader, CalldataReaderLib} from "src/types/CalldataReader.sol";
import {Pair, PairArray, PairLib} from "src/types/Pair.sol";
import {Asset as RefAsset, AssetLib as RefAssetLib} from "test/_reference/Asset.sol";
import {Asset, AssetArray, AssetLib} from "src/types/Asset.sol";
import {
    PoolConfigStoreLib, PoolConfigStore, MAX_FEE, ONE_E6
} from "src/libraries/PoolConfigStore.sol";
import {RayMathLib} from "../../src/libraries/RayMathLib.sol";

import {console} from "forge-std/console.sol";

/// @author philogy <https://github.com/philogy>
contract PairTest is BaseTest {
    using RayMathLib for uint256;
    using RefPairLib for RefPair[];
    using RefAssetLib for RefAsset[];

    function setUp() public {}

    function test_fuzzing_encodeDecodeSingle(
        address asset0,
        address asset1,
        uint256 price1Over0,
        uint16 tickSpacing,
        uint24 feeInE6
    ) public {
        vm.assume(asset0 != asset1);
        if (asset0 > asset1) (asset0, asset1) = (asset1, asset0);
        vm.assume(asset0 != address(0));
        feeInE6 = uint24(bound(feeInE6, 0, MAX_FEE));
        tickSpacing = u16(bound(tickSpacing, 1, type(uint16).max));
        price1Over0 = bound(price1Over0, 0, type(uint256).max / ONE_E6);

        PoolConfigStore store =
            PoolConfigStore.wrap(address(0)).setIntoNew(asset0, asset1, tickSpacing, feeInE6);

        RefAsset[] memory assets = new RefAsset[](2);
        assets[0].addr = asset0;
        assets[1].addr = asset1;
        RefPair[] memory pairs = new RefPair[](1);
        pairs[0] = RefPair(asset0, asset1, PriceAB.wrap(price1Over0));

        this._test_inner_encodeDecodeSingle(
            store,
            asset0,
            asset1,
            price1Over0,
            tickSpacing,
            feeInE6,
            bytes.concat(assets.encode(), pairs.encode(assets, PoolConfigStore.unwrap(store)))
        );
    }

    function _test_inner_encodeDecodeSingle(
        PoolConfigStore store,
        address asset0,
        address asset1,
        uint256 price1Over0,
        uint16 tickSpacing,
        uint24 feeInE6,
        bytes calldata data
    ) external view {
        CalldataReader reader = CalldataReaderLib.from(data);

        AssetArray assets;
        (reader, assets) = AssetLib.readFromAndValidate(reader);
        PairArray pairs;
        (reader, pairs) = PairLib.readFromAndValidate(reader, assets, store);
        reader.requireAtEndOf(data);

        {
            (address asset0Out, address asset1Out, int24 tickSpacingOut) =
                pairs.get(0).getPoolInfo();
            assertEq(asset0, asset0Out);
            assertEq(asset1, asset1Out);
            assertEq(int24(uint24(tickSpacing)), tickSpacingOut, "tick spacing");
        }

        {
            (address assetIn, address assetOut) = pairs.get(0).getAssets({zeroToOne: true});
            assertEq(assetIn, asset0);
            assertEq(assetOut, asset1);
            (assetIn, assetOut) = pairs.get(0).getAssets({zeroToOne: false});
            assertEq(assetIn, asset1);
            assertEq(assetOut, asset0);
        }

        {
            (address assetIn, address assetOut, uint256 priceOutVsIn) =
                pairs.get(0).getSwapInfo({zeroToOne: true});
            assertEq(assetIn, asset0);
            assertEq(assetOut, asset1);
            assertEq(priceOutVsIn, price1Over0 * (ONE_E6 - feeInE6) / ONE_E6, "price1Over0");

            (assetIn, assetOut, priceOutVsIn) = pairs.get(0).getSwapInfo({zeroToOne: false});
            assertEq(assetIn, asset1);
            assertEq(assetOut, asset0);
            assertEq(
                priceOutVsIn,
                price1Over0.invRayUnchecked() * (ONE_E6 - feeInE6) / ONE_E6,
                "price0Over1"
            );
        }
    }
}
