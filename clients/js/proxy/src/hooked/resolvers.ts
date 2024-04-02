import { Context, PublicKey } from '@metaplex-foundation/umi';
import { ResolvedAccounts, expectPublicKey } from '../generated';
import { findProxiedAssetPda } from '../pda';

export const resolveProxiedAsset = (
  context: Pick<Context, 'eddsa' | 'programs'>,
  accounts: ResolvedAccounts,
  _args: any,
  programId: PublicKey,
  isWritable: boolean
) => {
  if (accounts.stub?.value) {
    return {
      value: findProxiedAssetPda(context, {
        stub: expectPublicKey(accounts.stub.value),
      })[0],
      isWritable,
    };
  }
  return { value: programId, isWritable: false };
};
