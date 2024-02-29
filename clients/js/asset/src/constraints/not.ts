import { Serializer, struct, u32 } from '@metaplex-foundation/umi/serializers';
import { Constraint, getConstraintSerializer } from './constraint';
import {
  OperatorType,
  getOperatorTypeSerializer,
} from '../extensions/operatorType';

export type Not = {
  type: OperatorType;
  size: number;
  constraint: Constraint;
};

export const not = (constraint: Constraint): Not => ({
  type: OperatorType.Not,
  size: getConstraintSerializer().serialize(constraint).length,
  constraint,
});

export function getNotSerializer(): Serializer<Not, Not> {
  return {
    description: 'Not',
    fixedSize: null,
    maxSize: null,
    serialize: (value: Not) =>
      struct<Not>([
        ['type', getOperatorTypeSerializer()],
        ['size', u32()],
        ['constraint', getConstraintSerializer()],
      ]).serialize(value),
    deserialize: (buffer: Uint8Array) => {
      const [value, offset] = struct<Not>([
        ['type', getOperatorTypeSerializer()],
        ['size', u32()],
        ['constraint', getConstraintSerializer()],
      ]).deserialize(buffer);
      return [value, offset];
    },
  };
}
