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

export type OwnedBy = {
  type: 'OwnedBy';
  account: Account;
  owners: PublicKey[];
};

export const ownedBy = (
  account: Account,
  publicKeys: PublicKeyInput[]
): OwnedBy => ({
  type: 'OwnedBy',
  account,
  owners: publicKeys.map((owner) => toPublicKey(owner)),
});

export const getOwnedBySerializer = (): Serializer<OwnedBy> =>
  wrapSerializerInConstraintHeader(
    OperatorType.OwnedBy,
    struct([
      ['account', getAccountSerializer()],
      ['owners', array(publicKeySerializer(), { size: 'remainder' })],
    ])
  );
