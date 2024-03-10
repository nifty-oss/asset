import { Serializer, struct } from '@metaplex-foundation/umi/serializers';
import {
  Constraint,
  OperatorType,
  getConstraintSerializer,
  wrapSerializerInConstraintHeader,
} from '.';

export type Not = {
  type: 'Not';
  constraint: Constraint;
};

export const not = (constraint: Constraint): Not => ({
  type: 'Not',
  constraint,
});

export const getNotSerializer = (): Serializer<Not> =>
  wrapSerializerInConstraintHeader(
    OperatorType.Not,
    struct([['constraint', getConstraintSerializer()]])
  );
