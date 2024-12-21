import { parseArgs } from "util";
import { placeOrder } from "./order";

const { values } = parseArgs({
  args: Bun.argv,
  options: {
    type: {
      type: "string",
    },
    amount: {
      type: "string",
    },
    price: {
      type: "string",
    },
  },
  strict: true,
  allowPositionals: true,
});

if (!values.type || !values.amount || !values.price) {
  console.error("Missing required arguments");
  process.exit(1);
}
if (!["buy", "sell"].includes(values.type)) {
  console.error("Invalid type");
  process.exit(1);
}

const orderInput = {
  type: values.type as "buy" | "sell",
  amount: BigInt(values.amount!),
  price: BigInt(values.price!),
};

try {
  await placeOrder(orderInput);
} catch (e) {
  console.error(e);
  process.exit(1);
}
