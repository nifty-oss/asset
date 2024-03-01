import { Serializer, struct } from '@metaplex-foundation/umi/serializers';
import {
  Constraint,
  getConstraintSerializer,
  wrapSerializerInConstraintHeader,
} from '.';
import { OperatorType } from '../extensions';

export type Not = {
  type: OperatorType.Not;
  constraint: Constraint;
};

export const not = (constraint: Constraint): Not => ({
  type: OperatorType.Not,
  constraint,
});

export const getNotSerializer = (): Serializer<Not> =>
  wrapSerializerInConstraintHeader(
    OperatorType.Not,
    struct([['constraint', getConstraintSerializer()]])
  );
