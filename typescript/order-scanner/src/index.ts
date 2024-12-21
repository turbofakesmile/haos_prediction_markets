import { orderBookContract, startBlock, viemWalletClient } from "./config";

const POLLING_INTERVAL = 4000;

orderBookContract.watchEvent.OrderPlaced(
  {},
  {
    onLogs: async (logs) => {
      console.log(logs);
    },
    // onError: (error) => {
    //   console.error(error);
    // },
    fromBlock: 10600n,
    // pollingInterval: POLLING_INTERVAL,
  },
);

const blockInterval = 100n;

let currentBlock = startBlock;

while (true) {
  await new Promise((resolve) => setTimeout(resolve, POLLING_INTERVAL));

  const blockNumber = await viemWalletClient.getBlockNumber();
  let toBlock = currentBlock + blockInterval;
  if (toBlock > blockNumber) {
    toBlock = blockNumber;
  }
  if (currentBlock > blockNumber) {
    continue;
  }

  const logs = await orderBookContract.getEvents.OrderPlaced(
    {},
    {
      fromBlock: currentBlock,
      toBlock: toBlock,
    },
  );
  console.log(logs);
  currentBlock = toBlock + 1n;
}
