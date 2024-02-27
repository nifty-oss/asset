import { Serializer } from '@metaplex-foundation/umi/serializers';
import { OperatorType } from '../generated/types/operatorType';
import { PubkeyMatch, getPubkeyMatchSerializer } from './pubkeyMatch';
import { OwnedBy, getOwnedBySerializer } from './ownedBy';

export type Constraint = OwnedBy | PubkeyMatch;

export function getConstraintSerializer(): Serializer<Constraint, Constraint> {
  return {
    description: 'Constraint',
    fixedSize: null,
    maxSize: null,
    serialize: (value: Constraint) => {
      let buffer: Uint8Array;
      switch (value.type) {
        case OperatorType.OwnedBy: {
          const serializer = getOwnedBySerializer();
          let constraintBuffer = serializer.serialize(value as OwnedBy);

          // Remove the four bytes representing the array length to match the Rust type.
          const account = constraintBuffer.subarray(0, 8);
          const owners = constraintBuffer.subarray(12, constraintBuffer.length);
          constraintBuffer = new Uint8Array(account.length + owners.length);
          constraintBuffer.set(account);
          constraintBuffer.set(owners, account.length);

          const constraintSize = constraintBuffer.length;
          buffer = new Uint8Array(8 + constraintSize);
          const dataView = new DataView(buffer.buffer);
          dataView.setUint32(0, value.type, true);
          dataView.setUint32(4, constraintSize, true);
          buffer.set(constraintBuffer, 8);
          break;
        }
        case OperatorType.PubkeyMatch: {
          const serializer = getPubkeyMatchSerializer();
          let constraintBuffer = serializer.serialize(value as PubkeyMatch);
          // Remove the four bytes representing the array length to match the Rust type.
          const account = constraintBuffer.subarray(0, 8);
          const pubkeys = constraintBuffer.subarray(
            12,
            constraintBuffer.length
          );
          constraintBuffer = new Uint8Array(account.length + pubkeys.length);
          constraintBuffer.set(account);
          constraintBuffer.set(pubkeys, account.length);
          constraintBuffer = constraintBuffer.slice(4, constraintBuffer.length);

          const constraintSize = constraintBuffer.length;
          buffer = new Uint8Array(8 + constraintSize);
          const dataView = new DataView(buffer.buffer);
          dataView.setUint32(0, value.type);
          dataView.setUint32(4, constraintSize);
          buffer.set(constraintBuffer, 8);
          break;
        }
        default:
          throw new Error('Invalid constraint type');
      }
      return buffer;
    },
    deserialize: (buffer: Uint8Array) => {
      const dataView = new DataView(
        buffer.buffer,
        buffer.byteOffset,
        buffer.byteLength
      );
      const constraintType = dataView.getUint32(0) as OperatorType;
      const size = dataView.getUint32(4);

      const constraintData = buffer.slice(8, 8 + size);
      let constraint;
      switch (constraintType) {
        case OperatorType.OwnedBy: {
          const serializer = getOwnedBySerializer();
          constraint = serializer.deserialize(constraintData);
          break;
        }
        case OperatorType.PubkeyMatch: {
          const serializer = getPubkeyMatchSerializer();
          constraint = serializer.deserialize(constraintData);
          break;
        }
        default:
          throw new Error('Invalid constraint type');
      }

      return constraint;
    },
  };
}
