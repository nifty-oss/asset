import {
  Serializer,
  array,
  struct,
} from '@metaplex-foundation/umi/serializers';
import { Constraint, getConstraintSerializer } from './constraint';
import { OperatorType } from '../generated';

export type Or = {
  type: OperatorType.Or;
  constraints: Constraint[];
};

export type OrForSerialization = Omit<Or, 'type'>;

export const or = (constraints: Constraint[]): Or => ({
  type: OperatorType.Or,
  constraints,
});

export function getOrSerializer(): Serializer<Or, Or> {
  return {
    description: 'Or',
    fixedSize: null,
    maxSize: null,
    serialize: (value: Or) => {
      const valueForSerialization: OrForSerialization = {
        constraints: value.constraints,
      };
      return struct<OrForSerialization>([
        [
          'constraints',
          array(getConstraintSerializer(), { size: 'remainder' }),
        ],
      ]).serialize(valueForSerialization);
    },
    deserialize: (buffer: Uint8Array) => {
      // Slice off the type and size to get the actual constraint data.
      buffer = buffer.slice(8);
      const [valueForSerialization, offset] = struct<OrForSerialization>([
        [
          'constraints',
          array(getConstraintSerializer(), { size: 'remainder' }),
        ],
      ]).deserialize(buffer);
      const value: Or = {
        type: OperatorType.Or,
        ...valueForSerialization,
      };
      return [value, offset + 8];
    },
  };
}
