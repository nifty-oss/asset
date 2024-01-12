import { Serializer } from '@metaplex-foundation/umi/serializers';
import {
  Attributes,
  Creators,
  ExtensionType,
  Image,
  Links,
  getAttributesSerializer,
  getCreatorsSerializer,
  getImageSerializer,
  getLinksSerializer,
} from '../generated';

export * from './attributes';
export * from './creators';
export * from './image';
export * from './links';

export type Extension =
  | ({ type: ExtensionType.Attributes } & Attributes)
  | ({ type: ExtensionType.Creators } & Creators)
  | ({ type: ExtensionType.Image } & Image)
  | ({ type: ExtensionType.Links } & Links);

export const getExtensionSerializerFromType = <T extends Extension>(
  type: ExtensionType
): Serializer<T> =>
  ((): Serializer<any> => {
    switch (type) {
      case ExtensionType.Attributes:
        return getAttributesSerializer();
      case ExtensionType.Creators:
        return getCreatorsSerializer();
      case ExtensionType.Image:
        return getImageSerializer();
      case ExtensionType.Links:
        return getLinksSerializer();
      default:
        throw new Error(`Unknown extension type: ${type}`);
    }
  })() as Serializer<T>;
