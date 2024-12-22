// SPDX-License-Identifier: MIT
pragma solidity >=0.8.25 <0.9.0;

import { Test, console } from "forge-std/src/Test.sol";

import { EncryptedTokens } from "../src/FHERC20.sol";
import { OrderBook, Order, OrderInput } from "../src/OrderBook.sol";
import { FheEnabled } from "../util/FheHelper.sol";
import { Permission, PermissionHelper } from "../util/PermissionHelper.sol";

import { ebool, inEbool, euint32, inEuint32, FHE } from "@fhenixprotocol/contracts/FHE.sol";

/// @dev If this is your first time with Forge, read this tutorial in the Foundry Book:
/// https://book.getfoundry.sh/forge/writing-tests
contract TokenTest is Test, FheEnabled {
    EncryptedTokens internal token;
    OrderBook internal orderBook;

    address public owner;
    uint256 public ownerPrivateKey;

    uint256 private alicePrivateKey;
    address private alice;

    uint256 private bobPrivateKey;
    address private bob;

    /// @dev A function invoked before each test case is run.
    function setUp() public virtual {
        // Required to mock FHE operations - do not forget to call this function
        // *****************************************************
        initializeFhe();
        // *****************************************************

        alicePrivateKey = 0xA11CE;
        alice = vm.addr(alicePrivateKey);

        bobPrivateKey = 0xB0B;
        bob = vm.addr(bobPrivateKey);

        ownerPrivateKey = 0x0111AE4;
        owner = vm.addr(ownerPrivateKey);

        vm.startPrank(owner);

        // Instantiate the contract-under-test.
        token = new EncryptedTokens("");

        orderBook = new OrderBook(address(token));

        token.allowContract(address(orderBook));

        // token.mint("USD", alice, 1000);
        // token.mint("Yes", bob, 1000);

        vm.stopPrank();
    }

    function test_PlaceOrder() external {

        vm.startPrank(alice);
        uint256 id = 1;
        inEbool memory side = encryptBool(0);
        inEuint32 memory amount = encrypt32(50);
        inEuint32 memory price = encrypt32(100);

        orderBook.placeOrder(OrderInput(side, amount, price));

        (
            ebool orderSide, 
            euint32 orderAmount,
            euint32 orderPrice,
            address orderCreator
        ) = orderBook.orders(id);
        assertEq(orderSide.decrypt(), false);
        assertEq(orderAmount.decrypt(), 50);
        assertEq(orderPrice.decrypt(), 100);
        assertEq(orderCreator, alice);
    }

    function test_MatchOrersEqualPrice() external {

        {
            vm.startPrank(alice);
            uint256 id = 1;
            inEbool memory side = encryptBool(0);
            inEuint32 memory amount = encrypt32(50);
            inEuint32 memory price = encrypt32(100);

            orderBook.placeOrder(OrderInput(side, amount, price));

            (
                ebool orderSide, 
                euint32 orderAmount,
                euint32 orderPrice,
                address orderCreator
            ) = orderBook.orders(id);
            assertEq(orderSide.decrypt(), false);
            assertEq(orderAmount.decrypt(), 50);
            assertEq(orderPrice.decrypt(), 100);
            assertEq(orderCreator, alice);
            vm.stopPrank();
        }

        {
            vm.startPrank(bob);
            uint256 id = 2;
            inEbool memory side = encryptBool(1);
            inEuint32 memory amount = encrypt32(100);
            inEuint32 memory price = encrypt32(100);

            orderBook.placeOrder(OrderInput(side, amount, price));

            (
                ebool orderSide, 
                euint32 orderAmount,
                euint32 orderPrice,
                address orderCreator
            ) = orderBook.orders(id);
            assertEq(orderSide.decrypt(), true);
            assertEq(orderAmount.decrypt(), 100);
            assertEq(orderPrice.decrypt(), 100);
            assertEq(orderCreator, bob);
            vm.stopPrank();
        }

        orderBook.matchOrders(1, 2);

        {
            (
                ebool orderSide, 
                euint32 orderAmount,
                euint32 orderPrice,
                address orderCreator
            ) = orderBook.orders(1);
            assertEq(orderSide.decrypt(), false);
            assertEq(orderAmount.decrypt(), 0);
            assertEq(orderPrice.decrypt(), 100);
            assertEq(orderCreator, alice);
        }

        {
            (
                ebool orderSide, 
                euint32 orderAmount,
                euint32 orderPrice,
                address orderCreator
            ) = orderBook.orders(2);
            assertEq(orderSide.decrypt(), true);
            assertEq(orderAmount.decrypt(), 50);
            assertEq(orderPrice.decrypt(), 100);
            assertEq(orderCreator, bob);
        }
    }


    function test_MatchOrersDiffPrice() external {

        {
            vm.startPrank(alice);
            uint256 id = 1;
            inEbool memory side = encryptBool(0);
            inEuint32 memory amount = encrypt32(50);
            inEuint32 memory price = encrypt32(101);

            orderBook.placeOrder(OrderInput(side, amount, price));

            (
                ebool orderSide, 
                euint32 orderAmount,
                euint32 orderPrice,
                address orderCreator
            ) = orderBook.orders(id);
            assertEq(orderSide.decrypt(), false);
            assertEq(orderAmount.decrypt(), 50);
            assertEq(orderPrice.decrypt(), 101);
            assertEq(orderCreator, alice);
            vm.stopPrank();
        }

        {
            vm.startPrank(bob);
            uint256 id = 2;
            inEbool memory side = encryptBool(1);
            inEuint32 memory amount = encrypt32(100);
            inEuint32 memory price = encrypt32(100);

            orderBook.placeOrder(OrderInput(side, amount, price));

            (
                ebool orderSide, 
                euint32 orderAmount,
                euint32 orderPrice,
                address orderCreator
            ) = orderBook.orders(id);
            assertEq(orderSide.decrypt(), true);
            assertEq(orderAmount.decrypt(), 100);
            assertEq(orderPrice.decrypt(), 100);
            assertEq(orderCreator, bob);
            vm.stopPrank();
        }

        orderBook.matchOrders(1, 2);

        {
            (
                ebool orderSide, 
                euint32 orderAmount,
                euint32 orderPrice,
                address orderCreator
            ) = orderBook.orders(1);
            assertEq(orderSide.decrypt(), false);
            assertEq(orderAmount.decrypt(), 0);
            assertEq(orderPrice.decrypt(), 101);
            assertEq(orderCreator, alice);
        }

        {
            (
                ebool orderSide, 
                euint32 orderAmount,
                euint32 orderPrice,
                address orderCreator
            ) = orderBook.orders(2);
            assertEq(orderSide.decrypt(), true);
            assertEq(orderAmount.decrypt(), 50);
            assertEq(orderPrice.decrypt(), 100);
            assertEq(orderCreator, bob);
        }
    }

    function testFail_NonMatchingOrders() external {

        {
            vm.startPrank(alice);
            uint256 id = 1;
            inEbool memory side = encryptBool(0);
            inEuint32 memory amount = encrypt32(50);
            inEuint32 memory price = encrypt32(99);

            orderBook.placeOrder(OrderInput(side, amount, price));

            (
                ebool orderSide, 
                euint32 orderAmount,
                euint32 orderPrice,
                address orderCreator
            ) = orderBook.orders(id);
            assertEq(orderSide.decrypt(), false);
            assertEq(orderAmount.decrypt(), 50);
            assertEq(orderPrice.decrypt(), 99);
            assertEq(orderCreator, alice);
            vm.stopPrank();
        }

        {
            vm.startPrank(bob);
            uint256 id = 2;
            inEbool memory side = encryptBool(1);
            inEuint32 memory amount = encrypt32(100);
            inEuint32 memory price = encrypt32(100);

            orderBook.placeOrder(OrderInput(side, amount, price));

            (
                ebool orderSide, 
                euint32 orderAmount,
                euint32 orderPrice,
                address orderCreator
            ) = orderBook.orders(id);
            assertEq(orderSide.decrypt(), true);
            assertEq(orderAmount.decrypt(), 100);
            assertEq(orderPrice.decrypt(), 100);
            assertEq(orderCreator, bob);
            vm.stopPrank();
        }

        orderBook.matchOrders(1, 2);

        {
            (
                ebool orderSide, 
                euint32 orderAmount,
                euint32 orderPrice,
                address orderCreator
            ) = orderBook.orders(1);
            assertEq(orderSide.decrypt(), false);
            assertEq(orderAmount.decrypt(), 0);
            assertEq(orderPrice.decrypt(), 99);
            assertEq(orderCreator, alice);
        }

        {
            (
                ebool orderSide, 
                euint32 orderAmount,
                euint32 orderPrice,
                address orderCreator
            ) = orderBook.orders(2);
            assertEq(orderSide.decrypt(), true);
            assertEq(orderAmount.decrypt(), 50);
            assertEq(orderPrice.decrypt(), 100);
            assertEq(orderCreator, bob);
        }
    }

}
