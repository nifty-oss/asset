import { Serializer } from '@metaplex-foundation/umi/serializers';
import {
  Asset,
  Attributes,
  Blob,
  Creators,
  ExtensionType,
  Grouping,
  Links,
  Manager,
  Metadata,
  getAttributesSerializer,
  getBlobSerializer,
  getCreatorsSerializer,
  getGroupingSerializer,
  getLinksSerializer,
  getManagerSerializer,
  getMetadataSerializer,
} from '../generated';
import { Royalties, getRoyaltiesSerializer } from './royalties';

export * from './attributes';
export * from './blob';
export * from './creators';
export * from './grouping';
export * from './links';
export * from './metadata';
export * from './royalties';
export * from './manager';

export type TypedExtension =
  | ({ type: ExtensionType.Attributes } & Attributes)
  | ({ type: ExtensionType.Blob } & Blob)
  | ({ type: ExtensionType.Creators } & Creators)
  | ({ type: ExtensionType.Links } & Links)
  | ({ type: ExtensionType.Metadata } & Metadata)
  | ({ type: ExtensionType.Grouping } & Grouping)
  | ({ type: ExtensionType.Royalties } & Royalties)
  | ({ type: ExtensionType.Manager } & Manager);

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
