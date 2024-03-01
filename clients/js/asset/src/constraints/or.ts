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

export type Or = {
  type: OperatorType.Or;
  constraints: Constraint[];
};

export const or = (constraints: Constraint[]): Or => ({
  type: OperatorType.Or,
  constraints,
});

export const getOrSerializer = (): Serializer<Or> =>
  wrapSerializerInConstraintHeader(
    OperatorType.Or,
    struct([['constraints', array(getConstraintSerializer(), { size: u64() })]])
  );
