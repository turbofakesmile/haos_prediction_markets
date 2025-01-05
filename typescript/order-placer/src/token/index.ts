import { getContract } from "viem";
import { abi as tokenAbi } from "../_abi/token";
import { fhenixClient, globalConfig, viemWalletClient } from "../config";
import { encryptedToHex } from "../utils";

export type TokenOperation = {
  tokenAddress: string;
  amount?: bigint;
};

export async function mintToken({ tokenAddress, amount }: TokenOperation) {
  // Mock function for minting tokens
  console.log(`Minting ${amount} tokens at address ${tokenAddress}`);
  const contract = getContract({
    abi: tokenAbi,
    address: tokenAddress as `0x${string}`,
    client: viemWalletClient,
  });
  const myAddress = viemWalletClient.account.address;
  const txHash = await contract.write.mint([myAddress, amount!]);
  await viemWalletClient
    .waitForTransactionReceipt({ hash: txHash })
    .then(() => console.log("Tokens minted successfully"));
}

export async function getTokenBalance({ tokenAddress }: TokenOperation) {
  // Mock function for getting token balance
  console.log(`Getting balance for token at address ${tokenAddress}`);
  const contract = getContract({
    abi: tokenAbi,
    address: tokenAddress as `0x${string}`,
    client: viemWalletClient,
  });
  const myAddress = viemWalletClient.account.address;
  const balance = await contract.read.checkBalanceEncrypted([myAddress]);

  console.log(`Balance: ${balance}`);
  return balance;
}
