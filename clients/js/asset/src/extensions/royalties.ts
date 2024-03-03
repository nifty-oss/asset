import { Serializer, struct, u64 } from '@metaplex-foundation/umi/serializers';
import { Constraint, getConstraintSerializer } from '../constraints';
import { ExtensionType } from '../generated';
import { TypedExtension } from '.';

export type Royalties = {
  basisPoints: bigint | number;
  constraint: Constraint;
};

export function getRoyaltiesSerializer(): Serializer<Royalties> {
  return struct<Royalties>(
    [
      ['basisPoints', u64()],
      ['constraint', getConstraintSerializer()],
    ],
    { description: 'Royalties' }
  ) as Serializer<Royalties>;
}

export const royalties = (data: Royalties): TypedExtension => ({
  type: ExtensionType.Royalties,
  ...data,
});
