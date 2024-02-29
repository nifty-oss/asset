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

export type OwnedBy = {
  type: OperatorType;
  size: number;
  account: Account;
  owners: PublicKey[];
};

export const ownedBy = (
  account: Account,
  owners: PublicKeyInput[]
): OwnedBy => ({
  type: OperatorType.OwnedBy,
  size: ACCOUNT_SIZE + 32 * owners.length,
  account,
  owners: owners.map((pubkey) => toPublicKey(pubkey, true)),
});

export function getOwnedBySerializer(): Serializer<OwnedBy, OwnedBy> {
  return {
    description: 'OwnedBy',
    fixedSize: null,
    maxSize: null,
    serialize: (value: OwnedBy) =>
      struct<OwnedBy>([
        ['type', getOperatorTypeSerializer()],
        ['size', u32()],
        ['account', getAccountSerializer()],
        ['owners', array(publicKeySerializer(), { size: 'remainder' })],
      ]).serialize(value),
    deserialize: (buffer: Uint8Array, offset = 0) => {
      console.log('ownedBy buffer', buffer);
      console.log('ownedBy offset', offset);
      const dataView = new DataView(
        buffer.buffer,
        buffer.byteOffset,
        buffer.length
      );
      const size = dataView.getUint32(4, true);

      const numItems = size / 40;
      console.log('numItems', numItems);

      const [value, constraintOffset] = struct<OwnedBy>([
        ['type', getOperatorTypeSerializer()],
        ['size', u32()],
        ['account', getAccountSerializer()],
        ['owners', array(publicKeySerializer(), { size: numItems })],
      ]).deserialize(buffer, offset);
      console.log('ownedBy value', value);
      console.log('ownedBy constraintOffset', constraintOffset);
      return [value, constraintOffset];
    },
  };
}
