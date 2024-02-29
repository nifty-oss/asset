import {
  Serializer,
  scalarEnum,
  u32,
} from '@metaplex-foundation/umi/serializers';

export enum OperatorType {
  And,
  Not,
  Or,
  OwnedBy,
  PubkeyMatch,
}

export type OperatorTypeArgs = OperatorType;

export function getOperatorTypeSerializer(): Serializer<
  OperatorTypeArgs,
  OperatorType
> {
  return scalarEnum<OperatorType>(OperatorType, {
    description: 'OperatorType',
    size: u32(),
  }) as Serializer<OperatorTypeArgs, OperatorType>;
}
