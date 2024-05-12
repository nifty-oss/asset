/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import {
  Serializer,
  array,
  struct,
} from '@metaplex-foundation/umi/serializers';
import { Creator, CreatorArgs, getCreatorSerializer } from '.';

export type Creators = { values: Array<Creator> };

export type CreatorsArgs = { values: Array<CreatorArgs> };

export function getCreatorsSerializer(): Serializer<CreatorsArgs, Creators> {
  return struct<Creators>(
    [['values', array(getCreatorSerializer(), { size: 'remainder' })]],
    { description: 'Creators' }
  ) as Serializer<CreatorsArgs, Creators>;
}
