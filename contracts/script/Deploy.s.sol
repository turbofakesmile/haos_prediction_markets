// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.25 <0.9.0;

import { EncryptedToken } from "../src/FHERC20.sol";

import { OrderBook } from "../src/OrderBook.sol";

import { Script, console2 } from "forge-std/src/Script.sol";

/// @dev See the Solidity Scripting tutorial: https://book.getfoundry.sh/tutorials/solidity-scripting
contract Deploy is Script {
    function run() external {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");

        address owner = vm.addr(privateKey);
        console2.log("Using", owner, "as broadcaster");

        vm.startBroadcast(privateKey);

        EncryptedToken tokenA = new EncryptedToken();
        console2.log("EncryptedToken A deployed at", address(tokenA));

        EncryptedToken tokenB = new EncryptedToken();
        console2.log("EncryptedToken B deployed at", address(tokenB));

        OrderBook orderBook = new OrderBook(address(tokenA), address(tokenB));
        console2.log("OrderBook deployed at", address(orderBook));

        tokenA.allowContract(address(orderBook));
        tokenB.allowContract(address(orderBook));

        vm.stopBroadcast();
    }
}
