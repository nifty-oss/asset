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
  u32,
} from '@metaplex-foundation/umi/serializers';
import { Account, getAccountSerializer } from './account';
import {
  OperatorType,
  getOperatorTypeSerializer,
} from '../extensions/operatorType';

const ACCOUNT_SIZE = 8;

export type PubkeyMatch = {
  type: OperatorType;
  size: number;
  account: Account;
  pubkeys: PublicKey[];
};

export const pubkeyMatch = (
  account: Account,
  pubkeys: PublicKeyInput[]
): PubkeyMatch => ({
  type: OperatorType.PubkeyMatch,
  size: ACCOUNT_SIZE + 32 * pubkeys.length,
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
    serialize: (value: PubkeyMatch) =>
      struct<PubkeyMatch>([
        ['type', getOperatorTypeSerializer()],
        ['size', u32()],
        ['account', getAccountSerializer()],
        ['pubkeys', array(publicKeySerializer(), { size: 'remainder' })],
      ]).serialize(value),
    deserialize: (buffer: Uint8Array, offset = 0) => {
      const dataView = new DataView(
        buffer.buffer,
        buffer.byteOffset,
        buffer.length
      );
      const size = dataView.getUint32(offset + 4, true);

      const numItems = size / 40;

      const [value, constraintOffset] = struct<PubkeyMatch>([
        ['type', getOperatorTypeSerializer()],
        ['size', u32()],
        ['account', getAccountSerializer()],
        ['pubkeys', array(publicKeySerializer(), { size: numItems })],
      ]).deserialize(buffer, offset);
      return [value, constraintOffset];
    },
  };
}
