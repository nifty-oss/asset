import { Serializer, struct, u16 } from '@metaplex-foundation/umi/serializers';
import { Constraint, getConstraintSerializer } from '../constraints';
import { ExtensionType } from '../generated';
import { TypedExtension } from '.';

export type Royalties = {
  basisPoints: number;
  constraint: Constraint;
};

export type RoyaltiesArgs = {
  basisPoints: number;
  constraint: Constraint;
};

export function getRoyaltiesSerializer(): Serializer<RoyaltiesArgs, Royalties> {
  return struct<Royalties>(
    [
      ['basisPoints', u16()],
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
