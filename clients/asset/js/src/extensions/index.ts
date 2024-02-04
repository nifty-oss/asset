import { Serializer } from '@metaplex-foundation/umi/serializers';
import {
  Attributes,
  Creators,
  ExtensionType,
  Blob,
  Links,
  getAttributesSerializer,
  getBlobSerializer,
  getLinksSerializer,
  getCreatorsSerializer,
  Metadata,
  getMetadataSerializer,
} from '../generated';

export * from './attributes';
export * from './creators';
export * from './blob';
export * from './links';
export * from './metadata';

export type TypedExtension =
  | ({ type: ExtensionType.Attributes } & Attributes)
  | ({ type: ExtensionType.Blob } & Blob)
  | ({ type: ExtensionType.Creators } & Creators)
  | ({ type: ExtensionType.Links } & Links)
  | ({ type: ExtensionType.Metadata } & Metadata);

export const getExtensionSerializerFromType = <T extends TypedExtension>(
  type: ExtensionType
): Serializer<T> =>
  ((): Serializer<any> => {
    switch (type) {
      case ExtensionType.Attributes:
        return getAttributesSerializer();
      case ExtensionType.Blob:
        return getBlobSerializer();
      case ExtensionType.Creators:
        return getCreatorsSerializer();
      case ExtensionType.Links:
        return getLinksSerializer();
      case ExtensionType.Metadata:
        return getMetadataSerializer();
      default:
        throw new Error(`Unknown extension type: ${type}`);
    }
  })() as Serializer<T>;
