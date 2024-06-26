/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/kinobi-so/kinobi
 */

import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum Discriminator {
  Uninitialized,
  Asset,
}

export type DiscriminatorArgs = Discriminator;

export function getDiscriminatorSerializer(): Serializer<
  DiscriminatorArgs,
  Discriminator
> {
  return scalarEnum<Discriminator>(Discriminator, {
    description: 'Discriminator',
  }) as Serializer<DiscriminatorArgs, Discriminator>;
}
