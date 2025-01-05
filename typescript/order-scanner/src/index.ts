import { getOrderById } from "./order";

const server = Bun.serve({
  port: 3000,
  async fetch(req) {
    const url = new URL(req.url);

    // Handle GET /order/:id endpoint
    if (req.method === "GET" && url.pathname.startsWith("/order/")) {
      try {
        const orderId = url.pathname.split("/")[2]; // Get the ID from the URL
        if (!orderId) {
          return new Response(JSON.stringify({ error: "Order ID is required" }), {
            status: 400,
            headers: { "Content-Type": "application/json" },
          });
        }

        const order = await getOrderById(BigInt(orderId));
        return new Response(JSON.stringify(order), {
          status: 200,
          headers: { "Content-Type": "application/json" },
        });
      } catch (error) {
        console.error("Error fetching order:", error);
        return new Response(JSON.stringify({ error: "Failed to fetch order" }), {
          status: 500,
          headers: { "Content-Type": "application/json" },
        });
      }
    }

    // Handle 404 for unknown routes
    return new Response(JSON.stringify({ error: "Not found" }), {
      status: 404,
      headers: { "Content-Type": "application/json" },
    });
  },
});

console.log(`Server running at http://localhost:${server.port}`);
