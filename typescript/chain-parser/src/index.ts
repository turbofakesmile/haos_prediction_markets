import {
  bytesToHex,
  createPublicClient,
  createWalletClient,
  defineChain,
  getContract,
  http,
  publicActions,
} from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { EncryptedNumber, FhenixClient } from "fhenixjs";
import { JsonRpcProvider, Wallet } from "ethers";
import { abi as orderBookAbi } from "./abi/orderbook";

console.log("kek");

const privateKey = process.env.PRIVATE_KEY as `0x${string}`;

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

const ethersProvider = new JsonRpcProvider("https://api.nitrogen.fhenix.zone");

const ethersWallet = new Wallet(privateKey).connect(ethersProvider);

const walletClient = createWalletClient({
  account: privateKeyToAccount(privateKey),
  chain: fhenixChain,
  transport: http(),
}).extend(publicActions);

const fhenixClient = new FhenixClient({ provider: ethersProvider });

const encBool = await fhenixClient.encrypt_bool(true);

const contractAddress = "0x332aF9b971e33aF2f0d7056AE7245D64ccDF46a0";

const contract = getContract({
  abi: orderBookAbi,
  address: contractAddress,
  client: walletClient,
});

const encryptedToHex = (number: EncryptedNumber) => {
  return {
    data: bytesToHex(number.data),
    securityZone: number.securityZone,
  };
};

// const order = await contract.write.placeOrder([
//   {
//     side: encryptedToHex(await fhenixClient.encrypt_bool(false)),
//     amount: encryptedToHex(await fhenixClient.encrypt_uint32(50)),
//     price: encryptedToHex(await fhenixClient.encrypt_uint32(100)),
//   },
// ]);

// await walletClient.waitForTransactionReceipt({ hash: order }).then(console.log);

// console.log(await ethersWallet.provider.send("eth_getAccounts", []));

const permit = await fhenixClient.generatePermit(contractAddress, ethersProvider, ethersWallet);

const permission = fhenixClient.extractPermitPermission(permit);
const sealedResult = await contract.read.getOrderAmount([
  {
    publicKey: permission.publicKey as `0x${string}`,
    signature: permission.signature as `0x${string}`,
  },
  1n,
]);
console.log(fhenixClient.hasPermit(contractAddress, walletClient.account.address));
const result = await fhenixClient.unseal(contractAddress, sealedResult, walletClient.account.address);
console.log(result);
