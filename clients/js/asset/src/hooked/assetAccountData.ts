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

export type AssetAccountData = Omit<BaseAssetAccountData, 'delegate'> & {
  delegate: (Omit<Delegate, 'roles'> & { roles: DelegateRole[] }) | null;
  extensions: TypedExtension[];
};

export type AssetAccountDataArgs = Omit<
  BaseAssetAccountDataArgs,
  'delegate'
> & {
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

    return [
      {
        ...asset,
        delegate: asset.delegate.address ? asset.delegate : null,
        extensions,
      },
      finalOffset,
    ];
  },
});
