import {
  Serializer,
  array,
  struct,
} from '@metaplex-foundation/umi/serializers';
import { Constraint, getConstraintSerializer } from './constraint';
import { OperatorType } from '../generated';

export type And = {
  type: OperatorType.And;
  constraints: Constraint[];
};

export type AndForSerialization = Omit<And, 'type'>;

export const and = (constraints: Constraint[]): And => ({
  type: OperatorType.And,
  constraints,
});

export function getAndSerializer(): Serializer<And, And> {
  return {
    description: 'And',
    fixedSize: null,
    maxSize: null,
    serialize: (value: And) => {
      const valueForSerialization: AndForSerialization = {
        constraints: value.constraints,
      };
      return struct<AndForSerialization>([
        [
          'constraints',
          array(getConstraintSerializer(), { size: 'remainder' }),
        ],
      ]).serialize(valueForSerialization);
    },
    deserialize: (buffer: Uint8Array, offset = 0) => {
      // Slice off the type and size to get the actual constraint data.
      buffer = buffer.slice(8);
      const [valueForSerialization, constraintOffset] =
        struct<AndForSerialization>([
          [
            'constraints',
            array(getConstraintSerializer(), { size: 'remainder' }),
          ],
        ]).deserialize(buffer, offset);
      const value: And = {
        type: OperatorType.And,
        ...valueForSerialization,
      };
      return [value, constraintOffset + 8];
    },
  };
}
