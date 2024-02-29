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
    8
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
    deserialize: (buffer: Uint8Array) => {
      const [value, offset] = struct<Or>([
        ['type', getOperatorTypeSerializer()],
        ['size', u32()],
        [
          'constraints',
          array(getConstraintSerializer(), { size: 'remainder' }),
        ],
      ]).deserialize(buffer);
      return [value, offset + 8];
    },
  };
}
