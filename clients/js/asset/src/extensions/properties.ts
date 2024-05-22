import {
  Serializer,
  array,
  bool,
  string,
  struct,
  u64,
  u8,
} from '@metaplex-foundation/umi/serializers';
import { TypedExtension, getExtension } from '.';
import { Asset, ExtensionType, Type, getTypeSerializer } from '../generated';

type Property =
  | Omit<Text, 'type'>
  | Omit<Number, 'type'>
  | Omit<Boolean, 'type'>;

export const properties = (values: Property[]): TypedExtension => ({
  type: ExtensionType.Properties,
  values: values.map(
    (property) =>
      ({
        type: getTypeFromString(typeof property.value),
        ...property,
      } as TypedProperty)
  ),
});

type TypedPropertyfromEnum<T extends Type> = Extract<
  TypedProperty,
  { type: T }
>;

export function getProperty<T extends Type>(
  asset: Asset,
  name: string,
  type: T
): TypedPropertyfromEnum<T> | undefined {
  const extension = getExtension(asset, ExtensionType.Properties);

  if (!extension) {
    return undefined;
  }

  const property = extension.values.find(
    (p) => 'type' in p && p.type === type && p.name === name
  );

  return property ? (property as TypedPropertyfromEnum<T>) : undefined;
}

// Properties.

export type Properties = { values: Array<TypedProperty> };

export type PropertiesArgs = { values: Array<TypedProperty> };

export function getPropertiesSerializer(): Serializer<
  PropertiesArgs,
  Properties
> {
  return struct<Properties>(
    [['values', array(getPropertySerializer(), { size: 'remainder' })]],
    { description: 'Properties' }
  ) as Serializer<PropertiesArgs, Properties>;
}

export const getPropertySerializer = (): Serializer<TypedProperty> => ({
  description: 'TypedProperty',
  fixedSize: null,
  maxSize: null,
  serialize: (property: TypedProperty) =>
    getPropertySerializerFromType(property.type).serialize(property),
  deserialize: (buffer, offset = 0) => {
    const [, nameOffset] = string({ size: u8() }).deserialize(buffer, offset);
    const type = buffer[nameOffset] as Type;
    return getPropertySerializerFromType(type).deserialize(buffer, offset);
  },
});

export type TypedProperty =
  | ({ type: Type.Text } & Omit<Text, 'type'>)
  | ({ type: Type.Number } & Omit<Number, 'type'>)
  | ({ type: Type.Boolean } & Omit<Boolean, 'type'>);

export const getPropertySerializerFromType = <T extends TypedProperty>(
  type: Type
): Serializer<T> =>
  ((): Serializer<any> => {
    switch (type) {
      case Type.Text:
        return getTextSerializer();
      case Type.Number:
        return getNumberSerializer();
      case Type.Boolean:
        return getBooleanSerializer();
      default:
        throw new Error(`Unknown property type: ${type}`);
    }
  })() as Serializer<T>;

export const getTypeFromString = (type: String): Type => {
  switch (type) {
    case 'string':
      return Type.Text;
    case 'boolean':
      return Type.Boolean;
    default:
      return Type.Number;
  }
};

// Text property.

export type Text = { name: string; type: Type; value: string };

export type TextArgs = Text;

export function getTextSerializer(): Serializer<TextArgs, Text> {
  return struct<Text>(
    [
      ['name', string({ size: u8() })],
      ['type', getTypeSerializer()],
      ['value', string({ size: u8() })],
    ],
    { description: 'Text' }
  ) as Serializer<TextArgs, Text>;
}

// Number property.

export type Number = { name: string; type: Type; value: bigint };

export type NumberArgs = Number;

export function getNumberSerializer(): Serializer<NumberArgs, Number> {
  return struct<Number>(
    [
      ['name', string({ size: u8() })],
      ['type', getTypeSerializer()],
      ['value', u64()],
    ],
    { description: 'Number' }
  ) as Serializer<NumberArgs, Number>;
}

// Boolean property.

export type Boolean = { name: string; type: Type; value: boolean };

export type BooleanArgs = Boolean;

export function getBooleanSerializer(): Serializer<BooleanArgs, Boolean> {
  return struct<Boolean>(
    [
      ['name', string({ size: u8() })],
      ['type', getTypeSerializer()],
      ['value', bool()],
    ],
    { description: 'Boolean' }
  ) as Serializer<BooleanArgs, Boolean>;
}
