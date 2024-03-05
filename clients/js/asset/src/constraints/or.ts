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

export type Or = {
  type: 'Or';
  constraints: Constraint[];
};

export const or = (constraints: Constraint[]): Or => ({
  type: 'Or',
  constraints,
});

export const getOrSerializer = (): Serializer<Or> =>
  wrapSerializerInConstraintHeader(
    OperatorType.Or,
    struct([
      ['constraints', array(getConstraintSerializer(), { size: 'remainder' })],
    ])
  );
