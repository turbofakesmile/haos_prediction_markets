// SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.8.25 <0.9.0;

import { EncryptedTokens } from "../src/FHERC20.sol";

import { OrderBook } from "../src/OrderBook.sol";

import { Script, console2 } from "forge-std/src/Script.sol";

/// @dev See the Solidity Scripting tutorial: https://book.getfoundry.sh/tutorials/solidity-scripting
contract Deploy is Script {
    function run() external {
        uint256 privateKey = vm.envUint("PRIVATE_KEY");

        address owner = vm.addr(privateKey);
        console2.log("Using", owner, "as broadcaster");

        vm.startBroadcast(privateKey);

        EncryptedTokens token = new EncryptedTokens("");
        console2.log("EncryptedTokens deployed at", address(token));

        OrderBook orderBook = new OrderBook(address(token));
        console2.log("OrderBook deployed at", address(orderBook));

        token.allowContract(address(orderBook));

        vm.stopBroadcast();
    }
}
