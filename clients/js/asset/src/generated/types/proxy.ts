/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { PublicKey } from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  publicKey as publicKeySerializer,
  struct,
  u8,
} from '@metaplex-foundation/umi/serializers';

export type Proxy = { program: PublicKey; seeds: Array<number>; bump: number };

export type ProxyArgs = Proxy;

export function getProxySerializer(): Serializer<ProxyArgs, Proxy> {
  return struct<Proxy>(
    [
      ['program', publicKeySerializer()],
      ['seeds', array(u8(), { size: 32 })],
      ['bump', u8()],
    ],
    { description: 'Proxy' }
  ) as Serializer<ProxyArgs, Proxy>;
}
