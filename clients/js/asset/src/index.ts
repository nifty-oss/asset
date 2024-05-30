import {
  Context,
  PublicKey,
  gpaBuilder,
  publicKey,
} from '@metaplex-foundation/umi';
import {
  bool,
  publicKey as publicKeySerializer,
  string,
} from '@metaplex-foundation/umi/serializers';
import {
  DelegateArgs,
  DiscriminatorArgs,
  StandardArgs,
  StateArgs,
  getDelegateSerializer,
  getDiscriminatorSerializer,
  getStandardSerializer,
  getStateSerializer,
} from './generated';
import {
  InternalAsset as Asset,
  deserializeInternalAsset as deserializeAsset,
} from './generated/accounts/internalAsset';
import {
  NullablePublicKeyArgs,
  getNullablePublicKeySerializer,
} from './hooked';

export * from './allocate';
export * from './approve';
export * from './burn';
export * from './constraints';
export * from './create';
export * from './extensions';
export * from './generated';
export * from './group';
export * from './handover';
export * from './hooked';
export * from './initialize';
export * from './lock';
export * from './mint';
export * from './plugin';
export * from './remove';
export * from './resize';
export * from './revoke';
export * from './transfer';
export * from './ungroup';
export * from './unlock';
export * from './unverify';
export * from './update';
export * from './updateWithBuffer';
export * from './verify';
export * from './write';

export {
  InternalAsset as Asset,
  deserializeInternalAsset as deserializeAsset,
  fetchAllInternalAsset as fetchAllAsset,
  fetchInternalAsset as fetchAsset,
  safeFetchAllInternalAsset as safeFetchAllAsset,
  safeFetchInternalAsset as safeFetchAsset,
} from './generated/accounts/internalAsset';

export const SYSTEM_PROGRAM_ID = publicKey('11111111111111111111111111111111');

export function getAssetGpaBuilder(context: Pick<Context, 'rpc' | 'programs'>) {
  const programId = context.programs.getPublicKey(
    'asset',
    'AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73'
  );
  return gpaBuilder(context, programId)
    .registerFields<{
      discriminator: DiscriminatorArgs;
      state: StateArgs;
      standard: StandardArgs;
      mutable: boolean;
      owner: PublicKey;
      group: NullablePublicKeyArgs;
      authority: PublicKey;
      delegate: DelegateArgs;
      name: string;
    }>({
      discriminator: [0, getDiscriminatorSerializer()],
      state: [1, getStateSerializer()],
      standard: [2, getStandardSerializer()],
      mutable: [3, bool()],
      owner: [4, publicKeySerializer()],
      group: [36, getNullablePublicKeySerializer()],
      authority: [68, publicKeySerializer()],
      delegate: [100, getDelegateSerializer()],
      name: [133, string({ size: 35 })],
    })
    .deserializeUsing<Asset>((account) => deserializeAsset(account));
}
