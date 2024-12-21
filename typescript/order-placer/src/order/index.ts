import { getContract } from "viem";
import { abi as orderBookAbi } from "../_abi/orderbook";
import { fhenixClient, globalConfig, viemWalletClient } from "../config";
import { encryptedToHex } from "../utils";

export type OrderInputs = {
  type: "buy" | "sell";
  amount: bigint;
  price: bigint;
};

export async function placeOrder(orderInput: OrderInputs) {
  const contract = getContract({
    abi: orderBookAbi,
    address: globalConfig.contractAddress,
    client: viemWalletClient,
  });

  // 0 if buy, 1 if sell
  const rawSide = orderInput.type === "buy" ? false : true;

  const order = await contract.write.placeOrder([
    {
      side: encryptedToHex(await fhenixClient.encrypt_bool(rawSide)),
      amount: encryptedToHex(await fhenixClient.encrypt_uint32(Number(orderInput.amount))),
      price: encryptedToHex(await fhenixClient.encrypt_uint32(Number(orderInput.price))),
    },
  ]);
  await viemWalletClient.waitForTransactionReceipt({ hash: order }).then(console.log);
}
