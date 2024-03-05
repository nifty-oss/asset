import { Serializer, struct, u64 } from '@metaplex-foundation/umi/serializers';
import { Constraint, empty, getConstraintSerializer } from '../constraints';
import { ExtensionType } from '../generated';
import { TypedExtension } from '.';

export type Royalties = {
  basisPoints: bigint | number;
  constraint: Constraint;
};

export const royalties = (
  basisPoints: Royalties['basisPoints'],
  constraint: Royalties['constraint'] = empty()
): TypedExtension => ({
  type: ExtensionType.Royalties,
  basisPoints,
  constraint,
});

export function getRoyaltiesSerializer(): Serializer<Royalties> {
  return struct<Royalties>(
    [
      ['basisPoints', u64()],
      ['constraint', getConstraintSerializer()],
    ],
    { description: 'Royalties' }
  ) as Serializer<Royalties>;
}
