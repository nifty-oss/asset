import { Serializer, struct, u64 } from '@metaplex-foundation/umi/serializers';
import { Constraint, getConstraintSerializer } from '../constraints';
import { ExtensionType } from '../generated';
import { TypedExtension } from '.';

export type Royalties = {
  basisPoints: BigInt;
  constraint: Constraint;
};

export type RoyaltiesArgs = {
  basisPoints: BigInt;
  constraint: Constraint;
};

export function getRoyaltiesSerializer(): Serializer<RoyaltiesArgs, Royalties> {
  return struct<Royalties>(
    [
      ['basisPoints', u64() as unknown as Serializer<BigInt, BigInt>], // Fix: Explicitly type u64() serializer as Serializer<BigInt, BigInt>
      ['constraint', getConstraintSerializer()],
    ],
    { description: 'Royalties' }
  ) as Serializer<RoyaltiesArgs, Royalties>;
}

export const royalties = (data: RoyaltiesArgs): TypedExtension => ({
  type: ExtensionType.Royalties,
  basisPoints: data.basisPoints,
  constraint: data.constraint,
});
