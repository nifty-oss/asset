import { Serializer, struct } from '@metaplex-foundation/umi/serializers';
import { Constraint, getConstraintSerializer } from './constraint';
import { OperatorType } from '../generated';

export type Not = {
  type: OperatorType.Not;
  constraint: Constraint;
};

export const not = (constraint: Constraint): Not => ({
  type: OperatorType.Not,
  constraint,
});

export type NotForSerialization = Omit<Not, 'type'>;

export function getNotSerializer(): Serializer<Not, Not> {
  return {
    description: 'Not',
    fixedSize: null,
    maxSize: null,
    serialize: (value: Not) => {
      const valueForSerialization: NotForSerialization = {
        constraint: value.constraint,
      };
      return struct<NotForSerialization>([
        ['constraint', getConstraintSerializer()],
      ]).serialize(valueForSerialization);
    },
    deserialize: (buffer: Uint8Array) => {
      const [valueForSerialization, bytesRead] = struct<NotForSerialization>([
        ['constraint', getConstraintSerializer()],
      ]).deserialize(buffer);
      const value: Not = {
        type: OperatorType.Not,
        ...valueForSerialization,
      };
      return [value, bytesRead];
    },
  };
}
