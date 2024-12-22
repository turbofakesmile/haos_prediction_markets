import {
  ethersProvider,
  ethersWallet,
  fhenixClient,
  globalConfig,
  orderBookContract,
  viemWalletClient,
} from "../config";
const permit = await fhenixClient.generatePermit(globalConfig.contractAddress, ethersProvider, ethersWallet);
const permission = fhenixClient.extractPermitPermission(permit);

export type Order = {
  id: number;
  side: boolean;
  amount: number;
  price: number;
};

export async function getOrderById(orderId: bigint): Promise<Order> {
  const sealedResults = await orderBookContract.read.getOrder([
    {
      publicKey: permission.publicKey as `0x${string}`,
      signature: permission.signature as `0x${string}`,
    },
    orderId,
  ]);
  const side = fhenixClient.unseal(globalConfig.contractAddress, sealedResults[0], viemWalletClient.account.address);
  const amount = fhenixClient.unseal(globalConfig.contractAddress, sealedResults[1], viemWalletClient.account.address);
  const price = fhenixClient.unseal(globalConfig.contractAddress, sealedResults[2], viemWalletClient.account.address);
  return {
    id: Number(orderId),
    side: Boolean(side),
    amount: Number(amount),
    price: Number(price),
  };
}
