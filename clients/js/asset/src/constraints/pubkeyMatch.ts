import {
  PublicKey,
  PublicKeyInput,
  publicKey as toPublicKey,
} from '@metaplex-foundation/umi';
import {
  Serializer,
  array,
  publicKey as publicKeySerializer,
  struct,
} from '@metaplex-foundation/umi/serializers';
import { Account, getAccountSerializer } from './account';
import { OperatorType } from '../generated';

export type PubkeyMatch = {
  type: OperatorType.PubkeyMatch;
  account: Account;
  pubkeys: PublicKey[];
};

export const pubkeyMatch = (
  account: Account,
  pubkeys: PublicKeyInput[]
): PubkeyMatch => ({
  type: OperatorType.PubkeyMatch,
  account,
  pubkeys: pubkeys.map((pubkey) => toPublicKey(pubkey, true)),
});

export function getPubkeyMatchSerializer(): Serializer<
  PubkeyMatch,
  PubkeyMatch
> {
  return struct([
    ['account', getAccountSerializer()],
    ['pubkeys', array(publicKeySerializer())],
  ]);
}
