/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Serializer, struct, u64 } from '@metaplex-foundation/umi/serializers';
import {
  NullablePublicKey,
  NullablePublicKeyArgs,
  getNullablePublicKeySerializer,
} from '../../hooked';

export type Grouping = {
  size: bigint;
  maxSize: bigint;
  delegate: NullablePublicKey;
};

export type GroupingArgs = {
  size: number | bigint;
  maxSize: number | bigint;
  delegate: NullablePublicKeyArgs;
};

export function getGroupingSerializer(): Serializer<GroupingArgs, Grouping> {
  return struct<Grouping>(
    [
      ['size', u64()],
      ['maxSize', u64()],
      ['delegate', getNullablePublicKeySerializer()],
    ],
    { description: 'Grouping' }
  ) as Serializer<GroupingArgs, Grouping>;
}
