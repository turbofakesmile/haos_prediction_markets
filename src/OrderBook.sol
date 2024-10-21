// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import { EncryptedTokens } from "./FHERC20.sol";
import { 
    euint32 as encUint,
    inEuint32 as inEncUint,
    FHE,
    ebool,
    inEbool
} from "@fhenixprotocol/contracts/FHE.sol"; 
import { AccessControl } from "@openzeppelin/contracts/access/AccessControl.sol";

error FHERC20NotAuthorized();

struct Order {
	ebool side; // 0 if buy, 1 if sell
	encUint amount; // A if side=0, B if side=1
    encUint price;
	address creator;
}

struct OrderInput {
	inEbool side; // 0 if buy, 1 if sell
	inEncUint amount; // A if side=0, B if side=1
    inEncUint price;
	address creator;

}

contract OrderBook {

    mapping(uint256 => Order) public orders;
    mapping(uint256 => uint256) public ordersExist;

    EncryptedTokens public tokenA;
    EncryptedTokens public tokenB;

    constructor() {
        
    }
    
    function placeOrder(OrderInput calldata order, uint256 id) public {
        if (ordersExist[id] != 0) return;
        ordersExist[id] = 1;
        // EncryptedErc20.transfer(side, amountMake, address(this));
        orders[id] = Order(
            FHE.asEbool(order.side),
            FHE.asEuint32(order.amount),
            FHE.asEuint32(order.price),
            order.creator
        );
    }

    function matchOrders(uint256 takerOrderId, uint256 makerOrderId) public {
        require(ordersExist[takerOrderId] != 0, "Taker does not exist");
        require(ordersExist[makerOrderId] != 0, "Maker does not exist");

        Order memory takerOrder = orders[takerOrderId];
        Order memory makerOrder = orders[makerOrderId];

        ebool sidesDifferent = FHE.ne(takerOrder.side, makerOrder.side);
        ebool makerOrderNotFilled = FHE.gt(makerOrder.amount, FHE.asEuint32(0));
        ebool takerOrderNotFilled = FHE.gt(takerOrder.amount, FHE.asEuint32(0));

        ebool takerPriceEqual = FHE.eq(takerOrder.price, makerOrder.price);
        ebool takerPriceHigher = FHE.gt(takerOrder.price, makerOrder.price);

        encUint price = FHE.min(takerOrder.price, makerOrder.price);

        ebool orderCanBeFilled = FHE.or(
            takerPriceEqual, 
            FHE.xor(takerOrder.side, takerPriceHigher)
            /// if buy, taker price must be higher than maker price
            /// if sell, taker price must be lower than maker price
        );
        
        FHE.req(
            FHE.eq(
                FHE.and(
                    FHE.and(makerOrderNotFilled, takerOrderNotFilled),
                    FHE.and(sidesDifferent, orderCanBeFilled)
                ),
                FHE.asEbool(1)
            )
        );

        // // fill order

        encUint amount = FHE.min(takerOrder.amount, makerOrder.amount);
        takerOrder.amount = FHE.sub(takerOrder.amount, amount);
        makerOrder.amount = FHE.sub(makerOrder.amount, amount);

        // EncryptedErc20.transfer(side^1, takerTakeAmount, takerOrder.creator);
        // EncryptedErc20.transfer(side, takerMakeAmount, makerOrder.creator);

        // if taker is buy, taker takes A
        tokenA.transferEncrypted("Yes", takerOrder.creator, FHE.mul(amount, FHE.asEuint32(makerOrder.side)));
        // if taker is sell, maker takes A
        tokenA.transferEncrypted("Yes", makerOrder.creator, FHE.mul(amount, FHE.asEuint32(takerOrder.side)));
        // if taker is buy, maker takes B
        tokenB.transferEncrypted("Yes", makerOrder.creator, 
            FHE.mul(
                amount, 
                FHE.mul(price, FHE.asEuint32(makerOrder.side))
            )
        );
        // if taker is sell, taker takes B
        tokenB.transferEncrypted("Yes", takerOrder.creator, 
            FHE.mul(
                amount, 
                FHE.mul(price, FHE.asEuint32(takerOrder.side))
            )
        );

        orders[takerOrderId] = takerOrder;
        orders[makerOrderId] = makerOrder;

    }

}
