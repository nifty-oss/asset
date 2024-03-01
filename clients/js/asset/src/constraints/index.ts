import {
  Serializer,
  mergeBytes,
  scalarEnum,
  struct,
  u32,
  u64,
} from '@metaplex-foundation/umi/serializers';
import { OperatorType } from '../extensions';
import { And, getAndSerializer } from './and';
import { Not, getNotSerializer } from './not';
import { Or, getOrSerializer } from './or';
import { OwnedBy, getOwnedBySerializer } from './ownedBy';
import { PubkeyMatch, getPubkeyMatchSerializer } from './pubkeyMatch';

export * from './and';
export * from './not';
export * from './or';
export * from './ownedBy';
export * from './pubkeyMatch';

// -------------------//
// Constraint         //
// -------------------//

export type Constraint = And | Not | Or | OwnedBy | PubkeyMatch;

export const getConstraintSerializer = (): Serializer<Constraint> => ({
  description: 'Constraint',
  fixedSize: null,
  maxSize: null,
  serialize: (constraint: Constraint) =>
    getConstraintSerializerFromType(constraint.type).serialize(constraint),
  deserialize: (buffer, offset = 0) => {
    const type = buffer[offset] as OperatorType;
    const typeAsString = getOperatorTypeAsString(type);
    return getConstraintSerializerFromType(type).deserialize(buffer, offset);
  },
});

export const getConstraintSerializerFromType = <T extends Constraint>(
  type: T['type']
): Serializer<T> =>
  ((): Serializer<any> => {
    switch (type) {
      case OperatorType.And:
        return getAndSerializer();
      case OperatorType.Not:
        return getNotSerializer();
      case OperatorType.Or:
        return getOrSerializer();
      case OperatorType.OwnedBy:
        return getOwnedBySerializer();
      case OperatorType.PubkeyMatch:
        return getPubkeyMatchSerializer();
      default:
        throw new Error(`Unknown constraint type: ${type}`);
    }
  })() as Serializer<T>;

// -------------------//
// ConstraintHeader   //
// -------------------//

export type ConstraintHeader = { type: number; size: number };

export const getConstraintHeaderSerializer = (): Serializer<ConstraintHeader> =>
  struct([
    ['type', u32()],
    ['size', u32()],
  ]);

export const wrapSerializerInConstraintHeader = <
  T extends { type: OperatorType }
>(
  type: OperatorType,
  serializer: Serializer<Omit<T, 'type'>>
): Serializer<T> => {
  const HEADER_SIZE = 8; // 8 bytes for the constraint header
  const headerSerializer = getConstraintHeaderSerializer();
  return {
    description: getOperatorTypeAsString(type),
    fixedSize:
      serializer.fixedSize === null ? null : serializer.fixedSize + HEADER_SIZE,
    maxSize:
      serializer.maxSize === null ? null : serializer.maxSize + HEADER_SIZE,
    serialize: (rule: T): Uint8Array => {
      const serializedRule = serializer.serialize(rule);
      const serializedHeader = headerSerializer.serialize({
        type,
        size: serializedRule.length,
      });
      return mergeBytes([serializedHeader, serializedRule]);
    },
    deserialize: (buffer: Uint8Array, offset = 0): [T, number] => {
      const [header] = headerSerializer.deserialize(buffer, offset);
      offset += HEADER_SIZE;
      const slice = buffer.slice(offset, offset + header.size);
      const [rule] = serializer.deserialize(slice);
      return [{ ...rule, type } as T, offset + header.size];
    },
  };
};

export const getOperatorTypeAsString = (type: OperatorType) => {
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
