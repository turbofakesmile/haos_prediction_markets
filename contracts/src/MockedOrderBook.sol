// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

struct Order {
	bool side; // 0 if buy, 1 if sell
	uint32 amount; // A if side=0, B if side=1
    uint32 price;
	address creator;
}

struct OrderInput {
	bool side; // 0 if buy, 1 if sell
	uint32 amount; // A if side=0, B if side=1
    uint32 price;
}

event OrderPlaced(uint256 indexed id);
event OrderFilled(uint256 indexed id);
event OrdersMatched(uint256 indexed takerId, uint256 indexed makerId);

contract MockedOrderBook {

    mapping(uint256 => Order) public orders;
    mapping(uint256 => uint256) public ordersExist;

    uint256 public orderCount;

    string constant public tokenAName = "Yes";
    string constant public tokenBName = "USD";

    constructor() {}

    function getOrder(uint256 id) public view returns (bool, uint32, uint32) {
        require(ordersExist[id] != 0, "Order does not exist");
        return (
            orders[id].side,
            orders[id].amount,
            orders[id].price
        );
    }
    
    function placeOrder(OrderInput calldata order) public {
        orderCount++;
        uint256 id = orderCount;
        ordersExist[id] = 1;
        orders[id] = Order(
            order.side,
            order.amount,
            order.price,
            msg.sender
        );
        emit OrderPlaced(id);
    }

    function matchOrders(uint256 takerOrderId, uint256 makerOrderId) public {

        require(ordersExist[takerOrderId] != 0, "Taker does not exist");
        require(ordersExist[makerOrderId] != 0, "Maker does not exist");

        Order memory takerOrder = orders[takerOrderId];
        Order memory makerOrder = orders[makerOrderId];

        bool sidesDifferent = takerOrder.side != makerOrder.side;
        
        bool makerOrderNotFilled = makerOrder.amount > 0;
        bool takerOrderNotFilled = takerOrder.amount > 0;

        bool takerPriceEqual = takerOrder.price == makerOrder.price;
        bool takerPriceHigher = takerOrder.price > makerOrder.price;

        // uint32 price = takerOrder.price < makerOrder.price ? takerOrder.price : makerOrder.price;

        bool orderCanBeFilled = takerPriceEqual || (
            takerOrder.side != takerPriceHigher
        );

        require(
            sidesDifferent && makerOrderNotFilled && takerOrderNotFilled && orderCanBeFilled,
            "Orders cannot be filled"
        );
        // fill order

        uint32 amount = takerOrder.amount < makerOrder.amount ? takerOrder.amount : makerOrder.amount;
        takerOrder.amount -= amount;
        makerOrder.amount -= amount;

        orders[takerOrderId] = takerOrder;
        orders[makerOrderId] = makerOrder;

        emit OrderFilled(takerOrderId);
        emit OrderFilled(makerOrderId);
        emit OrdersMatched(takerOrderId, makerOrderId);
    }

}
