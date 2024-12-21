import { getEthersConfig, getFhenixClient, getViemClient } from "./fhenixChain";

if (!process.env.PRIVATE_KEY || !process.env.CONTRACT_ADDRESS) {
  throw new Error("Missing required environment variables");
}

const privateKey = process.env.PRIVATE_KEY as `0x${string}`;
const contractAddress = process.env.CONTRACT_ADDRESS as `0x${string}`;

export const globalConfig = {
  privateKey,
  contractAddress,
};

const { ethersProvider } = getEthersConfig(privateKey);
export const fhenixClient = getFhenixClient(ethersProvider);
export const viemWalletClient = getViemClient(privateKey);
