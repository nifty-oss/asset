import {
  Serializer,
  mergeBytes,
  scalarEnum,
  struct,
  u32,
  u64,
} from '@metaplex-foundation/umi/serializers';
import { And, getAndSerializer } from './and';
import { Not, getNotSerializer } from './not';
import { Or, getOrSerializer } from './or';
import { OwnedBy, getOwnedBySerializer } from './ownedBy';
import { PubkeyMatch, getPubkeyMatchSerializer } from './pubkeyMatch';
import { Empty, getEmptySerializer } from './empty';

export * from './and';
export * from './empty';
export * from './not';
export * from './or';
export * from './ownedBy';
export * from './pubkeyMatch';

// -------------------//
// Constraint         //
// -------------------//

export type Constraint = And | Not | Or | OwnedBy | PubkeyMatch | Empty;

export const getConstraintSerializer = (): Serializer<Constraint> => ({
  description: 'Constraint',
  fixedSize: null,
  maxSize: null,
  serialize: (constraint: Constraint) =>
    getConstraintSerializerFromType(constraint.type).serialize(constraint),
  deserialize: (buffer, offset = 0) => {
    const type = buffer[offset] as OperatorType;
    const typeAsString = getOperatorTypeAsString(type);
    return getConstraintSerializerFromType(typeAsString).deserialize(
      buffer,
      offset
    );
  },
});

export const getConstraintSerializerFromType = <T extends Constraint>(
  type: T['type']
): Serializer<T> =>
  ((): Serializer<any> => {
    switch (type) {
      case 'And':
        return getAndSerializer();
      case 'Not':
        return getNotSerializer();
      case 'Or':
        return getOrSerializer();
      case 'OwnedBy':
        return getOwnedBySerializer();
      case 'PubkeyMatch':
        return getPubkeyMatchSerializer();
      case 'Empty':
        return getEmptySerializer();
      default:
        throw new Error(`Unknown operator type: ${type}`);
    }
  })() as Serializer<T>;

// -------------------//
// OperatorType       //
// -------------------//

export enum OperatorType {
  And,
  Not,
  Or,
  OwnedBy,
  PubkeyMatch,
  Empty,
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

// -------------------//
// ConstraintHeader   //
// -------------------//

export type ConstraintHeader = { type: number; size: number };

export const getConstraintHeaderSerializer = (): Serializer<ConstraintHeader> =>
  struct([
    ['type', u32()],
    ['size', u32()],
  ]);

export const wrapSerializerInConstraintHeader = <T extends { type: string }>(
  type: OperatorType,
  serializer: Serializer<Omit<T, 'type'>>
): Serializer<T> => {
  const HEADER_SIZE = 8; // 8 bytes for the constraint header
  const typeAsString = getOperatorTypeAsString(type);
  const headerSerializer = getConstraintHeaderSerializer();
  return {
    description: typeAsString,
    fixedSize:
      serializer.fixedSize === null ? null : serializer.fixedSize + HEADER_SIZE,
    maxSize:
      serializer.maxSize === null ? null : serializer.maxSize + HEADER_SIZE,
    serialize: (constraint: T): Uint8Array => {
      const serializedConstaint = serializer.serialize(constraint);
      const serializedHeader = headerSerializer.serialize({
        type,
        size: serializedConstaint.length,
      });
      return mergeBytes([serializedHeader, serializedConstaint]);
    },
    deserialize: (buffer: Uint8Array, offset = 0): [T, number] => {
      const [header] = headerSerializer.deserialize(buffer, offset);
      offset += HEADER_SIZE;
      const slice = buffer.slice(offset, offset + header.size);
      const [constraint] = serializer.deserialize(slice);
      return [{ ...constraint, type: typeAsString } as T, offset + header.size];
    },
  };
};

export const getOperatorTypeAsString = (
  type: OperatorType
): Constraint['type'] => {
  switch (type) {
    case OperatorType.And:
      return 'And';
    case OperatorType.Not:
      return 'Not';
    case OperatorType.Or:
      return 'Or';
    case OperatorType.OwnedBy:
      return 'OwnedBy';
    case OperatorType.PubkeyMatch:
      return 'PubkeyMatch';
    case OperatorType.Empty:
      return 'Empty';
    default:
      throw new Error(`Unknown operator type: ${type}`);
  }
};

// -------------------//
// Account            //
// -------------------//

export enum Account {
  Asset,
  Authority,
  Recipient,
}

export type AccountArgs = Account;

export function getAccountSerializer(): Serializer<AccountArgs, Account> {
  return scalarEnum<Account>(Account, {
    size: u64(),
  }) as Serializer<AccountArgs, Account>;
}
