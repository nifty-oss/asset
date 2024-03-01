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
