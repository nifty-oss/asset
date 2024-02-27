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
import { OperatorType } from '../generated';
import { Account, getAccountSerializer } from './account';

export type OwnedBy = {
  type: OperatorType.OwnedBy;
  account: Account;
  owners: PublicKey[];
};

export const ownedBy = (
  account: Account,
  pubkeys: PublicKeyInput[]
): OwnedBy => ({
  type: OperatorType.OwnedBy,
  account,
  owners: pubkeys.map((pubkey) => toPublicKey(pubkey, true)),
});

export function getOwnedBySerializer(): Serializer<OwnedBy, OwnedBy> {
  return struct([
    ['account', getAccountSerializer()],
    ['owners', array(publicKeySerializer())],
  ]);
}
