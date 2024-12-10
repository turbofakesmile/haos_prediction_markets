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
import "@fhenixprotocol/contracts/access/Permissioned.sol";

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
}

contract OrderBook is Permissioned {

    mapping(uint256 => Order) public orders;
    mapping(uint256 => uint256) public ordersExist;

    EncryptedTokens public encryptedTokens;

    uint256 public orderCount;

    string constant public tokenAName = "Yes";
    string constant public tokenBName = "USD";

    constructor(address _encryptedTokens) {
        encryptedTokens = EncryptedTokens(_encryptedTokens);
    }

    function getOrderAmount(Permission calldata permission, uint256 id) public view returns (string memory) {
        require(ordersExist[id] != 0, "Order does not exist");
        return FHE.sealoutput(orders[id].amount, permission.publicKey);
    }
    
    function placeOrder(OrderInput calldata order) public {
        orderCount++;
        uint256 id = orderCount;
        ordersExist[id] = 1;
        orders[id] = Order(
            FHE.asEbool(order.side),
            FHE.asEuint32(order.amount),
            FHE.asEuint32(order.price),
            msg.sender
        );
        // // if order is buy, transfer B to this contract
        // encryptedTokens.transferFromEncrypted(
        //     msg.sender,
        //     address(this),
        //     tokenBName,
        //     FHE.mul(
        //         FHE.sub(FHE.asEuint32(1),
        //             FHE.asEuint32(
        //                 orders[id].side
        //             )
        //         ),
        //         orders[id].amount
        //     )
        // );
        // // if order is sell, transfer A to this contract
        // encryptedTokens.transferFromEncrypted(
        //     msg.sender,
        //     address(this),
        //     tokenAName,
        //     FHE.mul(
        //         FHE.asEuint32(
        //             orders[id].side
        //         ),
        //         orders[id].amount
        //     )
        // );
    }

    function matchOrders(uint256 takerOrderId, uint256 makerOrderId) public {

        require(ordersExist[takerOrderId] != 0, "Taker does not exist");
        require(ordersExist[makerOrderId] != 0, "Maker does not exist");

        Order memory takerOrder = orders[takerOrderId];
        Order memory makerOrder = orders[makerOrderId];

        ebool sidesDifferent = FHE.not(FHE.xor(takerOrder.side, makerOrder.side));

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
            FHE.and(
                FHE.and(makerOrderNotFilled, takerOrderNotFilled),
                FHE.and(sidesDifferent, orderCanBeFilled)
            )
        );
        // fill order

        encUint amount = FHE.min(takerOrder.amount, makerOrder.amount);
        takerOrder.amount = FHE.sub(takerOrder.amount, amount);
        makerOrder.amount = FHE.sub(makerOrder.amount, amount);

        // EncryptedErc20.transfer(side^1, takerTakeAmount, takerOrder.creator);
        // EncryptedErc20.transfer(side, takerMakeAmount, makerOrder.creator);

        // if taker is buy, taker takes A
        // encryptedTokens.transferFromEncrypted(
        //     address(this),
        //     takerOrder.creator, 
        //     tokenAName, 
        //     FHE.mul(amount, FHE.asEuint32(makerOrder.side))
        // );
        // // if taker is sell, maker takes A
        // encryptedTokens.transferFromEncrypted(
        //     address(this),
        //     makerOrder.creator, 
        //     tokenAName, 
        //     FHE.mul(amount, FHE.asEuint32(takerOrder.side))
        // );
        // // if taker is buy, maker takes B
        // encryptedTokens.transferFromEncrypted(
        //     address(this),
        //     makerOrder.creator, 
        //     tokenBName, 
        //     FHE.mul(
        //         amount, 
        //         FHE.mul(price, FHE.asEuint32(makerOrder.side))
        //     )
        // );
        // // if taker is sell, taker takes B
        // encryptedTokens.transferFromEncrypted(
        //     address(this),
        //     takerOrder.creator, 
        //     tokenBName, 
        //     FHE.mul(
        //         amount, 
        //         FHE.mul(price, FHE.asEuint32(takerOrder.side))
        //     )
        // );

        orders[takerOrderId] = takerOrder;
        orders[makerOrderId] = makerOrder;

    }

}
