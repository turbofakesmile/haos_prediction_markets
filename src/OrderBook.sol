// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import { FHERC20 } from "@fhenixprotocol/contracts/experimental/token/FHERC20/FHERC20.sol";
import { 
    euint128 as encUint,
    inEuint128 as inEncUint,
    FHE,
    ebool
} from "@fhenixprotocol/contracts/FHE.sol";
import { AccessControl } from "@openzeppelin/contracts/access/AccessControl.sol";

error FHERC20NotAuthorized();

struct Order {
	encUint side; // 0 if buy, 1 if sell
	encUint amountTake; // A if side=0, B if side=1
	encUint amountMake; // B if side=0, A if side=0
	address creator;
}

struct OrderInput {
	inEncUint side; // 0 if buy, 1 if sell
	inEncUint amountTake; // A if side=0, B if side=1
	inEncUint amountMake; // B if side=0, A if side=0
	address creator;

}

contract OrderBook {

    mapping(uint256 => Order) public orders;
    mapping(uint256 => uint256) public ordersExist;

    constructor() {
        
    }
    
    function placeOrder(OrderInput calldata order, uint256 id) public {
        if (ordersExist[id] != 0) return;
        ordersExist[id] = 1;
        // EncryptedErc20.transfer(side, amountMake, address(this));
        orders[id] = Order(
            FHE.asEuint128(order.side),
            FHE.asEuint128(order.amountTake),
            FHE.asEuint128(order.amountMake),
            order.creator
        );
    }

    function matchOrders(uint256 takerOrderId, uint256 makerOrderId) public {
        require(ordersExist[takerOrderId] != 0, "Taker does not exist");
        require(ordersExist[makerOrderId] != 0, "Maker does not exist");

        Order memory takerOrder = orders[takerOrderId];
        Order memory makerOrder = orders[makerOrderId];

        ebool sidesDifferent = FHE.ne(takerOrder.side, makerOrder.side);
        ebool orderNotFilled = FHE.ne(takerOrder.side, makerOrder.side);
        // FHE.assert(orders[takerOrderId].side != orders[makerOrderId].side);
        // FHE.assert(orders[takerOrderId].amount > 0);
        // FHE.assert(orders[makerOrderId].amount > 0);

        // price1 = amountTake/amountMake
        // price2 = amountTake/amountMake

        // // if taker (priceTaker) is buy, maker (priceMaker) is sell:
        // // assert(priceTaker >= priceMaker)

        // // if taker (priceTaker) is sell, maker (priceMaker) is buy:
        // // assert(priceTaker <= priceMaker)

        // // priceTaker - priceMaker >= 0  if buy
        // // priceTaker - priceMaker <= 0, if sell

        // // using unsigned nature of encrypted amount, we can check
        // assert(priceTaker-priceMaker >= takerOrder.side* (MAX_UINT-MAX_AMOUNT));

        // // fill order

        // takerTakeAmount = min(takerOrder.takeAmount, makerOrder.makeAmount)
        // takerMakeAmount = min(takerOrder.makeAmount, makerOrder.takeAmount);

        // EncryptedErc20.transfer(side^1, takerTakeAmount, takerOrder.creator);
        // EncryptedErc20.transfer(side, takerMakeAmount, makerOrder.creator);

        // takerOrder.takeAmount -= takerTakeAmount;
        // makerOrder.makeAmount -= takerTakeAmount;

        // takerOrder.makeAmount -= takerMakeAmount;
        // makerOrder.takeAmount -= takerMakeAmount;

    }

}
