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
import {
  Account,
  getAccountSerializer,
  wrapSerializerInConstraintHeader,
} from '.';
import { OperatorType } from '../extensions';

export type PubkeyMatch = {
  type: OperatorType.PubkeyMatch;
  account: Account;
  pubkeys: PublicKey[];
};

export const pubkeyMatch = (
  account: Account,
  publicKeys: PublicKeyInput[]
): PubkeyMatch => ({
  type: OperatorType.PubkeyMatch,
  account,
  pubkeys: publicKeys.map((address) => toPublicKey(address)),
});

export const getPubkeyMatchSerializer = (): Serializer<PubkeyMatch> =>
  wrapSerializerInConstraintHeader(
    OperatorType.PubkeyMatch,
    struct([
      ['account', getAccountSerializer()],
      ['pubkeys', array(publicKeySerializer(), { size: 'remainder' })],
    ])
  );
