import { Serializer, struct } from '@metaplex-foundation/umi/serializers';
import { OperatorType, wrapSerializerInConstraintHeader } from '.';

export type Empty = {
  type: 'Empty';
};

export const empty = (): Empty => ({
  type: 'Empty',
});

export const getEmptySerializer = (): Serializer<Empty> =>
  wrapSerializerInConstraintHeader(OperatorType.Empty, struct([]));
