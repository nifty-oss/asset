import {
  Serializer,
  array,
  struct,
  u64,
} from '@metaplex-foundation/umi/serializers';
import {
  Constraint,
  getConstraintSerializer,
  wrapSerializerInConstraintHeader,
} from '.';
import { OperatorType } from '../extensions';

export type And = {
  type: OperatorType.And;
  constraints: Constraint[];
};

export const and = (constraints: Constraint[]): And => ({
  type: OperatorType.And,
  constraints,
});

export const getAndSerializer = (): Serializer<And> =>
  wrapSerializerInConstraintHeader(
    OperatorType.And,
    struct([['constraints', array(getConstraintSerializer(), { size: u64() })]])
  );
