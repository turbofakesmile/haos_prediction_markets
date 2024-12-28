// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.25 <0.9.0;

import { MockedOrderBook } from "../src/MockedOrderBook.sol";

import { Script, console2 } from "forge-std/src/Script.sol";

/// @dev See the Solidity Scripting tutorial: https://book.getfoundry.sh/tutorials/solidity-scripting
contract DeployMock is Script {
    function run() external {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");

        address owner = vm.addr(privateKey);
        console2.log("Using", owner, "as broadcaster");

        vm.startBroadcast(privateKey);

        MockedOrderBook orderBook = new MockedOrderBook();
        console2.log("OrderBook deployed at", address(orderBook));

        vm.stopBroadcast();
    }
}
