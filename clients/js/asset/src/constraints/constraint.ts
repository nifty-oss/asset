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
          buffer = serializer.serialize(value as OwnedBy);
          break;
        }
        case OperatorType.PubkeyMatch: {
          const serializer = getPubkeyMatchSerializer();
          buffer = serializer.serialize(value as PubkeyMatch);
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
