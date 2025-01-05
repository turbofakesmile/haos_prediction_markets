import { Command } from "commander";
import { placeOrder } from "./order";
import { getTokenBalance, mintToken } from "./token";

interface OrderOptions {
  type: string;
  amount: string;
  price: string;
}

interface TokenOptions {
  address: string;
  amount: string;
}

interface BalanceOptions {
  address: string;
}

const program = new Command();

program.name("order-placer").description("CLI tool for interacting with the prediction markets").version("1.0.0");

program
  .command("place-order")
  .description("Place a new order")
  .requiredOption("-t, --type <type>", "Order type (buy/sell)")
  .requiredOption("-a, --amount <amount>", "Order amount")
  .requiredOption("-p, --price <price>", "Order price")
  .action(async (options: OrderOptions) => {
    if (!["buy", "sell"].includes(options.type)) {
      console.error("Invalid type. Must be 'buy' or 'sell'");
      process.exit(1);
    }
    try {
      await placeOrder({
        type: options.type as "buy" | "sell",
        amount: BigInt(options.amount),
        price: BigInt(options.price),
      });
    } catch (e) {
      console.error(e);
      process.exit(1);
    }
  });

program
  .command("mint-token")
  .description("Mint new tokens")
  .requiredOption("-a, --address <address>", "Token address")
  .requiredOption("-m, --amount <amount>", "Amount to mint")
  .action(async (options: TokenOptions) => {
    try {
      await mintToken({
        tokenAddress: options.address,
        amount: BigInt(options.amount),
      });
    } catch (e) {
      console.error(e);
      process.exit(1);
    }
  });

program
  .command("view-balance")
  .description("View token balance")
  .requiredOption("-a, --address <address>", "Token address")
  .action(async (options: BalanceOptions) => {
    try {
      await getTokenBalance({
        tokenAddress: options.address,
      });
    } catch (e) {
      console.error(e);
      process.exit(1);
    }
  });

program.parse();
