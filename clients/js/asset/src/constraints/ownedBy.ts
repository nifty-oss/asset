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

export type OwnedByForSerialization = Omit<OwnedBy, 'type'>;

export const ownedBy = (
  account: Account,
  pubkeys: PublicKeyInput[]
): OwnedBy => ({
  type: OperatorType.OwnedBy,
  account,
  owners: pubkeys.map((pubkey) => toPublicKey(pubkey, true)),
});

export function getOwnedBySerializer(): Serializer<OwnedBy, OwnedBy> {
  return {
    description: 'OwnedBy',
    fixedSize: null,
    maxSize: null,
    serialize: (value: OwnedBy) => {
      const valueForSerialization: OwnedByForSerialization = {
        account: value.account,
        owners: value.owners,
      };
      return struct<OwnedByForSerialization>([
        ['account', getAccountSerializer()],
        ['owners', array(publicKeySerializer(), { size: 'remainder' })],
      ]).serialize(valueForSerialization);
    },
    deserialize: (buffer: Uint8Array, offset = 0) => {
      const dataView = new DataView(
        buffer.buffer,
        buffer.byteOffset,
        buffer.length
      );
      const size = dataView.getUint32(offset + 4, true);

      // Slice off the type and size to get the actual constraint data.
      const numItems = (size - 8) / 32;

      buffer = buffer.slice(8);

      const [valueForSerialization, constraintOffset] =
        struct<OwnedByForSerialization>([
          ['account', getAccountSerializer()],
          ['owners', array(publicKeySerializer(), { size: numItems })],
        ]).deserialize(buffer, offset);
      const value: OwnedBy = {
        type: OperatorType.OwnedBy,
        ...valueForSerialization,
      };
      return [value, constraintOffset + 8];
    },
  };
}
