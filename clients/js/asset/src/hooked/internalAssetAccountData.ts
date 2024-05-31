import { Serializer } from '@metaplex-foundation/umi/serializers';
import { TypedExtension, getExtensionSerializerFromType } from '../extensions';
import {
  Delegate,
  DelegateRole,
  Discriminator,
  ExtensionType,
} from '../generated';
import {
  InternalAssetAccountData as BaseInternalAssetAccountData,
  InternalAssetAccountDataArgs as BaseInternalAssetAccountDataArgs,
  getInternalAssetAccountDataSerializer as getBaseInternalAssetAccountDataSerializer,
} from '../generated/types/internalAssetAccountData';
import { getExtensionHeaderSerializer } from '../generated/types/extensionHeader';

export type InternalAssetAccountData = Omit<
  BaseInternalAssetAccountData,
  'delegate'
> & {
  delegate: (Omit<Delegate, 'roles'> & { roles: DelegateRole[] }) | null;
  extensions: TypedExtension[];
};

export type InternalAssetAccountDataArgs = Omit<
  BaseInternalAssetAccountDataArgs,
  'delegate'
> & {
  delegate: (Omit<Delegate, 'roles'> & { roles: DelegateRole[] }) | null;
  extensions: TypedExtension[];
};

export const getInternalAssetAccountDataSerializer = (): Serializer<
  InternalAssetAccountDataArgs,
  InternalAssetAccountData
> => ({
  description: 'AssetAccountData',
  fixedSize: null,
  maxSize: null,
  serialize: () => {
    throw new Error('Operation not supported.');
  },
  deserialize: (
    buffer: Uint8Array,
    offset = 0
  ): [InternalAssetAccountData, number] => {
    // Account.
    const [asset, assetOffset] =
      getBaseInternalAssetAccountDataSerializer().deserialize(buffer, offset);
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

      if (type === ExtensionType.None) {
        break;
      } else if (ExtensionType[type]) {
        let endOffset = headerOffset + header.length;
        if (type === ExtensionType.Metadata) {
          // backwards compatibility for metadata extension: there is now an 'imageUrl'
          // on the extension, so we use the extra padding to simulate having it for
          // assets created before the change.
          endOffset = header.boundary;
        }
        const [extension] = getExtensionSerializerFromType(type).deserialize(
          buffer.subarray(headerOffset, endOffset)
        );
        extensions.push({ ...extension, type } as TypedExtension);
      }

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
