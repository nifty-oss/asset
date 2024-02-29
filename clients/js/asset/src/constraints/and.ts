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

export type And = {
  type: OperatorType;
  size: number;
  constraints: Constraint[];
};

export const and = (constraints: Constraint[]): And => ({
  type: OperatorType.And,
  size: constraints.reduce(
    (acc, constraint) =>
      acc + getConstraintSerializer().serialize(constraint).length,
    0
  ),
  constraints,
});

export function getAndSerializer(): Serializer<And, And> {
  return {
    description: 'And',
    fixedSize: null,
    maxSize: null,
    serialize: (value: And) =>
      struct<And>([
        ['type', getOperatorTypeSerializer()],
        ['size', u32()],
        [
          'constraints',
          array(getConstraintSerializer(), { size: 'remainder' }),
        ],
      ]).serialize(value),
    deserialize: (buffer: Uint8Array, offset = 0) => {
      console.log('and buffer', buffer);
      console.log('and offset', offset);
      const dataView = new DataView(
        buffer.buffer,
        buffer.byteOffset,
        buffer.length
      );
      const size = dataView.getUint32(offset + 4, true);
      console.log('size', size);

      const numItems = size / 48;
      console.log('numItems', numItems);

      const [value, constraintOffset] = struct<And>([
        ['type', getOperatorTypeSerializer()],
        ['size', u32()],
        [
          'constraints',
          array(getConstraintSerializer(), { size: 'remainder' }),
        ],
      ]).deserialize(buffer, offset);
      return [value, constraintOffset + 8];
    },
  };
}
