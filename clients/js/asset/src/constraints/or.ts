import {
  Serializer,
  array,
  struct,
  u32,
} from '@metaplex-foundation/umi/serializers';
import { Constraint, getConstraintSerializer } from './constraint';
import {
  OperatorType,
  getOperatorTypeSerializer,
} from '../extensions/operatorType';

export type Or = {
  type: OperatorType;
  size: number;
  constraints: Constraint[];
};

export const or = (constraints: Constraint[]): Or => ({
  type: OperatorType.Or,
  size: constraints.reduce(
    (acc, constraint) =>
      acc + getConstraintSerializer().serialize(constraint).length,
    0
  ),
  constraints,
});

export function getOrSerializer(): Serializer<Or, Or> {
  return {
    description: 'Or',
    fixedSize: null,
    maxSize: null,
    serialize: (value: Or) =>
      struct<Or>([
        ['type', getOperatorTypeSerializer()],
        ['size', u32()],
        [
          'constraints',
          array(getConstraintSerializer(), { size: 'remainder' }),
        ],
      ]).serialize(value),
    deserialize: (buffer: Uint8Array, offset = 0) => {
      const [type, o] = getOperatorTypeSerializer().deserialize(buffer, offset);
      offset = o;
      const [size, o2] = u32().deserialize(buffer, offset);
      offset = o2;

      const constraints = [];

      while (offset < buffer.length) {
        const [constraint, constraintOffset] =
          getConstraintSerializer().deserialize(buffer, offset);
        constraints.push(constraint);
        offset += constraintOffset;
      }

      const value = {
        type,
        size,
        constraints,
      };

      return [value, offset];
    },
  };
}
