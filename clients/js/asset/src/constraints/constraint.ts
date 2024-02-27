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
          const constraintBuffer = serializer.serialize(value as OwnedBy);
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
          const constraintBuffer = serializer.serialize(value as PubkeyMatch);
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
        buffer.length
      );
      const constraintType = dataView.getUint32(8, true) as OperatorType;
      const size = dataView.getUint32(12, true);

      const constraintData = buffer.slice(16, size + 16);
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
