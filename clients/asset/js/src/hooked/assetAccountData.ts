import { Serializer } from '@metaplex-foundation/umi/serializers';
import {
  AssetAccountData as BaseAssetAccountData,
  AssetAccountDataArgs as BaseAssetAccountDataArgs,
  getAssetAccountDataSerializer as getBaseAssetAccountDataSerializer,
} from '../generated/types/assetAccountData';
import { Discriminator, ExtensionType } from '../generated';
import { getExtensionHeaderSerializer } from '../generated/types/extensionHeader';
import { Extension, getExtensionSerializerFromType } from '../extensions';

export type AssetAccountData = BaseAssetAccountData & {
  extensions: Extension[];
};

export type AssetAccountDataArgs = BaseAssetAccountDataArgs & {
  extensions: Extension[];
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
    const extensions: Extension[] = [];

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
      extensions.push({ ...extension, type } as Extension);

      finalOffset = header.boundary;
    }

    return [
      {
        ...asset,
        extensions,
      },
      finalOffset,
    ];
  },
});
