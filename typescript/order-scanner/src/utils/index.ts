import type { EncryptedNumber } from "fhenixjs";
import { bytesToHex } from "viem";

export const encryptedToHex = (number: EncryptedNumber) => {
  return {
    data: bytesToHex(number.data),
    securityZone: number.securityZone,
  };
};
