import { Serializer } from '@metaplex-foundation/umi/serializers';
import {
  Attributes,
  Creators,
  ExtensionType,
  Blob,
  Links,
  getAttributesSerializer,
  getCreatorsSerializer,
  getBlobSerializer,
  getLinksSerializer,
} from '../generated';

export * from './attributes';
export * from './creators';
export * from './blob';
export * from './links';

export type Extension =
  | ({ type: ExtensionType.Attributes } & Attributes)
  | ({ type: ExtensionType.Blob } & Blob)
  | ({ type: ExtensionType.Creators } & Creators)
  | ({ type: ExtensionType.Links } & Links);

export const getExtensionSerializerFromType = <T extends Extension>(
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
      default:
        throw new Error(`Unknown extension type: ${type}`);
    }
  })() as Serializer<T>;
