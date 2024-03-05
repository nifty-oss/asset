import {
  Serializer,
  array,
  struct,
} from '@metaplex-foundation/umi/serializers';
import {
  Constraint,
  OperatorType,
  getConstraintSerializer,
  wrapSerializerInConstraintHeader,
} from '.';

export type And = {
  type: 'And';
  constraints: Constraint[];
};

export const and = (constraints: Constraint[]): And => ({
  type: 'And',
  constraints,
});

export const getAndSerializer = (): Serializer<And> =>
  wrapSerializerInConstraintHeader(
    OperatorType.And,
    struct([
      ['constraints', array(getConstraintSerializer(), { size: 'remainder' })],
    ])
  );
