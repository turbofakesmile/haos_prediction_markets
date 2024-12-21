import { Wallet } from "ethers";
import { JsonRpcProvider } from "ethers";
import { FhenixClient } from "fhenixjs";
import { createWalletClient, defineChain, http, publicActions } from "viem";
import { privateKeyToAccount } from "viem/accounts";

const fhenixChain = defineChain({
  id: 8008148,
  name: "fhenix helium",
  nativeCurrency: {
    decimals: 18,
    name: "Ether",
    symbol: "ETH",
  },
  rpcUrls: {
    default: {
      http: ["https://api.nitrogen.fhenix.zone"],
    },
  },
});

export function getEthersConfig(privateKey: string) {
  const ethersProvider = new JsonRpcProvider("https://api.nitrogen.fhenix.zone");

  const ethersWallet = new Wallet(privateKey).connect(ethersProvider);
  return { ethersProvider, ethersWallet };
}

export function getFhenixClient(ethersProvider: JsonRpcProvider) {
  return new FhenixClient({ provider: ethersProvider });
}

export const getViemClient = (privateKey: `0x${string}`) => {
  const walletClient = createWalletClient({
    account: privateKeyToAccount(privateKey),
    chain: fhenixChain,
    transport: http(),
  }).extend(publicActions);
  return walletClient;
};

export type ViemWalletClient = ReturnType<typeof getViemClient>;
