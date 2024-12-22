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
  amount: number;
};

export async function getOrderById(orderId: bigint): Promise<Order> {
  const sealedResult = await orderBookContract.read.getOrderAmount([
    {
      publicKey: permission.publicKey as `0x${string}`,
      signature: permission.signature as `0x${string}`,
    },
    orderId,
  ]);
  const result = fhenixClient.unseal(globalConfig.contractAddress, sealedResult, viemWalletClient.account.address);
  return {
    id: Number(orderId),
    amount: Number(result),
  };
}
