import { orderBookUrl } from "../config";
import type { Order } from "../order";

// post order to orderbook
export async function postOrder(order: Order) {
  const response = await fetch(`${orderBookUrl}/new_order`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(order),
  });
  if (!response.ok) {
    throw new Error(`Failed to post order: ${response.statusText}`);
  }
}
