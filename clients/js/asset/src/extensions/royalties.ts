import { Serializer, struct, u64 } from '@metaplex-foundation/umi/serializers';
import { Constraint, getConstraintSerializer } from '../constraints';
import { ExtensionType } from '../generated';
import { TypedExtension } from '.';

export type Royalties = {
  basisPoints: BigInt;
  constraint: Constraint;
};

export type RoyaltiesArgs = Royalties;

export function getRoyaltiesSerializer(): Serializer<RoyaltiesArgs, Royalties> {
  return struct<Royalties>(
    [
      ['basisPoints', u64() as Serializer<BigInt | number, BigInt>],
      ['constraint', getConstraintSerializer()],
    ],
    { description: 'Royalties' }
  ) as Serializer<RoyaltiesArgs, Royalties>;
}

export const royalties = (data: Royalties): TypedExtension => ({
  type: ExtensionType.Royalties,
  ...data,
});
