import { Serializer } from '@metaplex-foundation/umi/serializers';
import { Asset } from '..';
import {
  Attributes,
  Blob,
  Bucket,
  Creators,
  ExtensionType,
  Grouping,
  Links,
  Manager,
  Metadata,
  Proxy,
  getAttributesSerializer,
  getBlobSerializer,
  getBucketSerializer,
  getCreatorsSerializer,
  getGroupingSerializer,
  getLinksSerializer,
  getManagerSerializer,
  getMetadataSerializer,
  getProxySerializer,
} from '../generated';
import { Royalties, getRoyaltiesSerializer } from './royalties';
import { Properties, getPropertiesSerializer } from './properties';

export * from './attributes';
export * from './blob';
export * from './bucket';
export * from './creators';
export * from './grouping';
export * from './links';
export * from './manager';
export * from './metadata';
export * from './properties';
export * from './royalties';

export type TypedExtension =
  | ({ type: ExtensionType.Attributes } & Attributes)
  | ({ type: ExtensionType.Blob } & Blob)
  | ({ type: ExtensionType.Creators } & Creators)
  | ({ type: ExtensionType.Links } & Links)
  | ({ type: ExtensionType.Metadata } & Metadata)
  | ({ type: ExtensionType.Grouping } & Grouping)
  | ({ type: ExtensionType.Royalties } & Royalties)
  | ({ type: ExtensionType.Manager } & Manager)
  | ({ type: ExtensionType.Proxy } & Proxy)
  | ({ type: ExtensionType.Properties } & Properties)
  | ({ type: ExtensionType.Bucket } & Bucket);

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
      case ExtensionType.Grouping:
        return getGroupingSerializer();
      case ExtensionType.Royalties:
        return getRoyaltiesSerializer();
      case ExtensionType.Manager:
        return getManagerSerializer();
      case ExtensionType.Proxy:
        return getProxySerializer();
      case ExtensionType.Properties:
        return getPropertiesSerializer();
      case ExtensionType.Bucket:
        return getBucketSerializer();
      default:
        throw new Error(`Unknown extension type: ${type}`);
    }
  })() as Serializer<T>;

type TypedExtensionfromEnum<T extends ExtensionType> = Extract<
  TypedExtension,
  { type: T }
>;

export function getExtension<T extends ExtensionType>(
  asset: Asset,
  extensionType: T
): TypedExtensionfromEnum<T> | undefined {
  const extension = asset.extensions.find(
    (e) => 'type' in e && e.type === extensionType
  );

  return extension ? (extension as TypedExtensionfromEnum<T>) : undefined;
}
