import { getEthersConfig, getFhenixClient, getViemClient } from "./fhenixChain";
import { abi as orderBookAbi } from "../_abi/orderbook";
import { getContract } from "viem";

if (!process.env.PRIVATE_KEY || !process.env.CONTRACT_ADDRESS) {
  throw new Error("Missing required environment variables");
}

const privateKey = process.env.PRIVATE_KEY as `0x${string}`;
const contractAddress = process.env.CONTRACT_ADDRESS as `0x${string}`;

export const globalConfig = {
  privateKey,
  contractAddress,
};

const { ethersProvider, ethersWallet } = getEthersConfig(privateKey);
export { ethersProvider, ethersWallet };

export const fhenixClient = getFhenixClient(ethersProvider);
export const viemWalletClient = getViemClient(privateKey);

export const orderBookContract = getContract({
  abi: orderBookAbi,
  address: globalConfig.contractAddress,
  client: viemWalletClient,
});
