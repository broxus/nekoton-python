import { Transaction } from 'everscale-inpage-provider';

import { getSavedProviderKey } from '../providers/useProvider';

export const testContract = {
  contracts: {
    'EVER Wallet': {
      address: ``,
      dublicateAddress: ``,
    },
    // 'VENOM Wallet': {
    //   address: `0:8978c67c7bb707773e91b4f3a0c33b24346910dd0a906ea6806501fd4a8fec61`,
    //   dublicateAddress: `0:a494e9f70a4b641ae4c19b91371a60dacf2ecfc1bb5ab46d7ac3dbb257ebd553`,
    // },
  },
  getAddress: () => {
    const providerKey = getSavedProviderKey();
    if (providerKey) {
      return (testContract.contracts as Record<string, any>)[providerKey];
    }

    return testContract.contracts['EVER Wallet'];
  },
  ABI: {
    'ABI version': 2,
  } as const,
  base64: `te6ccgECHgEAA/EA...AAA==`,
  boc: `te6ccgECJAEABTQAAm/...9sIDAuNjYuMAAA`,
};

export const loadBase64FromFile = async (filePath: string) => {
  try {
    const response = await fetch(filePath);
    if (!response.ok) {
      throw new Error(`Failed to load file: ${response.statusText}`);
    }
    const text = await response.text();

    return text.split('\n').join('');
  } catch (e) {
    return undefined;
  }
};

export const tryLoadTvcFromFile = async (filePath: string) => {
  try {
    const response = await fetch(filePath);
    if (!response.ok) {
      throw new Error(`Failed to load file: ${response.statusText}`);
    }

    return await response.text();
  } catch (e) {
    return undefined;
  }
};

export const toNano = (value: number) => String(value * 1e9);

export const errorExtractor = async <
  T extends { transaction: Transaction; output?: Record<string, unknown> },
>(
  transactionResult: Promise<T>
): Promise<T> => {
  return transactionResult.then(res => {
    if (res.transaction.aborted) {
      throw {
        message: `Transaction aborted with code ${res.transaction.exitCode}`,
        name: 'TransactionAborted',
        transaction: res,
      };
    }

    return res;
  });
};

// export const txResultToast = (txResult: Transaction) => {
//   if (txResult.aborted) {
//     toast(`Transaction aborted with code ${txResult.exitCode}`, 0);
//   } else {
//     toast(`Message sent`, 1);
//   }
// };
export * from './toast';
