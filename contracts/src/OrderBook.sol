// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import { EncryptedToken } from "./FHERC20.sol";
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

event OrderPlaced(uint256 indexed id);
event OrderFilled(uint256 indexed id);
event OrdersMatched(uint256 indexed takerId, uint256 indexed makerId);

contract OrderBook is Permissioned {
    mapping(uint256 => Order) public orders;
    mapping(uint256 => uint256) public ordersExist;

    EncryptedToken public tokenA;
    EncryptedToken public tokenB;

    uint256 public orderCount;

    string constant public tokenAName = "Yes";
    string constant public tokenBName = "USD";

    constructor(address _tokenA, address _tokenB) {
        tokenA = EncryptedToken(_tokenA);
        tokenB = EncryptedToken(_tokenB);
    }

    function getOrder(Permission calldata permission, uint256 id) public view returns (
        string memory,
        string memory,
        string memory
    ) {
        require(ordersExist[id] != 0, "Order does not exist");
        return (
            FHE.sealoutput(orders[id].side, permission.publicKey),
            FHE.sealoutput(orders[id].amount, permission.publicKey),
            FHE.sealoutput(orders[id].price, permission.publicKey)
        );
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
        
        // If order is buy, transfer B (USD) to this contract
        tokenB.transferFromEncrypted(
            msg.sender,
            address(this),
            FHE.mul(
                FHE.sub(
                    FHE.asEuint32(1),
                    FHE.asEuint32(orders[id].side)
                ),
                FHE.mul(orders[id].amount, orders[id].price)
            )
        );
        
        // If order is sell, transfer A (Yes tokens) to this contract
        tokenA.transferFromEncrypted(
            msg.sender,
            address(this),
            FHE.mul(
                FHE.asEuint32(orders[id].side),
                orders[id].amount
            )
        );
        
        emit OrderPlaced(id);
    }

    function matchOrders(uint256 takerOrderId, uint256 makerOrderId) public {
        require(ordersExist[takerOrderId] != 0, "Taker does not exist");
        require(ordersExist[makerOrderId] != 0, "Maker does not exist");

        Order memory takerOrder = orders[takerOrderId];
        Order memory makerOrder = orders[makerOrderId];

        ebool sidesDifferent = FHE.xor(takerOrder.side, makerOrder.side);
        ebool makerOrderNotFilled = FHE.gt(makerOrder.amount, FHE.asEuint32(0));
        ebool takerOrderNotFilled = FHE.gt(takerOrder.amount, FHE.asEuint32(0));
        ebool takerPriceEqual = FHE.eq(takerOrder.price, makerOrder.price);
        ebool takerPriceHigher = FHE.gt(takerOrder.price, makerOrder.price);

        encUint price = FHE.min(takerOrder.price, makerOrder.price);

        ebool orderCanBeFilled = FHE.or(
            takerPriceEqual,
            /// if buy, taker price must be higher than maker price
            /// if sell, taker price must be lower than maker price
            FHE.xor(takerOrder.side, takerPriceHigher)
        );
        
        FHE.req(
            FHE.and(
                FHE.and(makerOrderNotFilled, takerOrderNotFilled),
                FHE.and(sidesDifferent, orderCanBeFilled)
            )
        );

        encUint amount = FHE.min(takerOrder.amount, makerOrder.amount);
        takerOrder.amount = FHE.sub(takerOrder.amount, amount);
        makerOrder.amount = FHE.sub(makerOrder.amount, amount);

        // If taker is buy, transfer A (Yes tokens) to taker
        tokenA.transferFromEncrypted(
            address(this),
            takerOrder.creator,
            FHE.mul(amount, FHE.sub(FHE.asEuint32(1), FHE.asEuint32(takerOrder.side)))
        );
        
        // If taker is sell, transfer A (Yes tokens) to maker
        tokenA.transferFromEncrypted(
            address(this),
            makerOrder.creator,
            FHE.mul(amount, FHE.asEuint32(takerOrder.side))
        );
        
        // If taker is buy, transfer B (USD) to maker
        tokenB.transferFromEncrypted(
            address(this),
            makerOrder.creator,
            FHE.mul(
                amount,
                FHE.mul(
                    price,
                    FHE.sub(FHE.asEuint32(1), FHE.asEuint32(takerOrder.side))
                )
            )
        );
        
        // If taker is sell, transfer B (USD) to taker
        tokenB.transferFromEncrypted(
            address(this),
            takerOrder.creator,
            FHE.mul(
                amount,
                FHE.mul(price, FHE.asEuint32(takerOrder.side))
            )
        );

        orders[takerOrderId] = takerOrder;
        orders[makerOrderId] = makerOrder;

        emit OrderFilled(takerOrderId);
        emit OrderFilled(makerOrderId);
        emit OrdersMatched(takerOrderId, makerOrderId);
    }
}
