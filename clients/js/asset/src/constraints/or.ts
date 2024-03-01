import {
  Serializer,
  array,
  struct,
} from '@metaplex-foundation/umi/serializers';
import {
  Constraint,
  getConstraintSerializer,
  wrapSerializerInConstraintHeader,
} from '.';
import { OperatorType } from '../extensions';

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
