import {
  ethersProvider,
  ethersWallet,
  fhenixClient,
  globalConfig,
  orderBookContract,
  viemWalletClient,
} from "../config";

export async function getOrderById(orderId: bigint) {
  const permit = await fhenixClient.generatePermit(globalConfig.contractAddress, ethersProvider, ethersWallet);

  const permission = fhenixClient.extractPermitPermission(permit);
  const sealedResult = await orderBookContract.read.getOrderAmount([
    {
      publicKey: permission.publicKey as `0x${string}`,
      signature: permission.signature as `0x${string}`,
    },
    orderId,
  ]);
  console.log(fhenixClient.hasPermit(globalConfig.contractAddress, viemWalletClient.account.address));
  const result = await fhenixClient.unseal(
    globalConfig.contractAddress,
    sealedResult,
    viemWalletClient.account.address,
  );
  console.log(result);
}
