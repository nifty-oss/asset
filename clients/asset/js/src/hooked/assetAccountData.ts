import { PublicKey, defaultPublicKey } from '@metaplex-foundation/umi';
import { Serializer } from '@metaplex-foundation/umi/serializers';
import { TypedExtension, getExtensionSerializerFromType } from '../extensions';
import {
  Delegate,
  DelegateRole,
  Discriminator,
  ExtensionType,
} from '../generated';
import {
  AssetAccountData as BaseAssetAccountData,
  AssetAccountDataArgs as BaseAssetAccountDataArgs,
  getAssetAccountDataSerializer as getBaseAssetAccountDataSerializer,
} from '../generated/types/assetAccountData';
import { getExtensionHeaderSerializer } from '../generated/types/extensionHeader';
import { isActive } from '../types';

export type AssetAccountData = Omit<
  BaseAssetAccountData,
  'group' | 'delegate'
> & {
  group: PublicKey | null;
  delegate: (Omit<Delegate, 'roles'> & { roles: DelegateRole[] }) | null;
  extensions: TypedExtension[];
};

export type AssetAccountDataArgs = Omit<
  BaseAssetAccountDataArgs,
  'group' | 'delegate'
> & {
  group: PublicKey | null;
  delegate: (Omit<Delegate, 'roles'> & { roles: DelegateRole[] }) | null;
  extensions: TypedExtension[];
};

export const getAssetAccountDataSerializer = (): Serializer<
  AssetAccountDataArgs,
  AssetAccountData
> => ({
  description: 'AssetAccountData',
  fixedSize: null,
  maxSize: null,
  serialize: () => {
    throw new Error('Operation not supported.');
  },
  deserialize: (buffer: Uint8Array, offset = 0): [AssetAccountData, number] => {
    // Account.
    const [asset, assetOffset] =
      getBaseAssetAccountDataSerializer().deserialize(buffer, offset);
    if (asset.discriminator !== Discriminator.Asset) {
      throw new Error(
        `Expected an Asset account, got account discriminator: ${asset.discriminator}`
      );
    }

    let finalOffset = assetOffset;
    const extensions: TypedExtension[] = [];

    // Extensions.
    while (finalOffset < buffer.length) {
      const [header, headerOffset] = getExtensionHeaderSerializer().deserialize(
        buffer,
        finalOffset
      );
      const type = header.kind as ExtensionType;
      const [extension] = getExtensionSerializerFromType(type).deserialize(
        buffer.subarray(headerOffset, headerOffset + header.length)
      );
      extensions.push({ ...extension, type } as TypedExtension);

      finalOffset = header.boundary;
    }

    // Delegate.
    const address =
      asset.delegate.address !== defaultPublicKey()
        ? asset.delegate.address
        : null;
    let roles: DelegateRole[] = [];

    if (address) {
      roles = Object.keys(DelegateRole)
        .filter((key): key is keyof typeof DelegateRole =>
          Number.isNaN(Number(key))
        )
        .map((key) => DelegateRole[key])
        .filter((value) => isActive(asset.delegate, value));
    }

    return [
      {
        ...asset,
        group: asset.group !== defaultPublicKey() ? asset.group : null,
        delegate: address ? { address, roles } : null,
        extensions,
      },
      finalOffset,
    ];
  },
});
