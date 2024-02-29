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

export type PubkeyMatchForSerialization = Omit<PubkeyMatch, 'type'>;

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
  return {
    description: 'PubkeyMatch',
    fixedSize: null,
    maxSize: null,
    serialize: (value: PubkeyMatch) => {
      const valueForSerialization: PubkeyMatchForSerialization = {
        account: value.account,
        pubkeys: value.pubkeys,
      };
      return struct<PubkeyMatchForSerialization>([
        ['account', getAccountSerializer()],
        ['pubkeys', array(publicKeySerializer(), { size: 'remainder' })],
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

      // Slice off the type and size to get the actual constraint data.
      buffer = buffer.slice(8);
      const [valueForSerialization, constraintOffset] =
        struct<PubkeyMatchForSerialization>([
          ['account', getAccountSerializer()],
          ['pubkeys', array(publicKeySerializer(), { size: numItems })],
        ]).deserialize(buffer, offset);
      const value: PubkeyMatch = {
        type: OperatorType.PubkeyMatch,
        ...valueForSerialization,
      };
      return [value, constraintOffset + 7];
    },
  };
}
