import { Context, PublicKey } from '@metaplex-foundation/umi';
import { ResolvedAccounts, expectPublicKey } from '../generated';
import { findBridgeAssetPda } from '../pda';

export const resolveBridgeAsset = (
  context: Pick<Context, 'eddsa' | 'programs'>,
  accounts: ResolvedAccounts,
  _args: any,
  programId: PublicKey,
  isWritable: boolean
) => {
  if (accounts.mint?.value) {
    return {
      value: findBridgeAssetPda(context, {
        mint: expectPublicKey(accounts.mint.value),
      })[0],
      isWritable,
    };
  }
  return { value: programId, isWritable: false };
};
