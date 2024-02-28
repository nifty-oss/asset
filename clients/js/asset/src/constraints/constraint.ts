import { Serializer } from '@metaplex-foundation/umi/serializers';
import { OperatorType } from '../generated/types/operatorType';
import { PubkeyMatch, getPubkeyMatchSerializer } from './pubkeyMatch';
import { OwnedBy, getOwnedBySerializer } from './ownedBy';
import { Or, getOrSerializer } from './or';
import { Not, getNotSerializer } from './not';
import { And, getAndSerializer } from './and';

export type Constraint = And | Not | Or | OwnedBy | PubkeyMatch;

export function getConstraintSerializer(): Serializer<Constraint, Constraint> {
  return {
    description: 'Constraint',
    fixedSize: null,
    maxSize: null,
    serialize: (value: Constraint) => {
      let constraintBuffer: Uint8Array;
      switch (value.type) {
        case OperatorType.And: {
          const serializer = getAndSerializer();
          constraintBuffer = serializer.serialize(value as And);
          break;
        }
        case OperatorType.Not: {
          const serializer = getNotSerializer();
          constraintBuffer = serializer.serialize(value as Not);
          break;
        }
        case OperatorType.Or: {
          const serializer = getOrSerializer();
          constraintBuffer = serializer.serialize(value as Or);
          break;
        }
        case OperatorType.OwnedBy: {
          const serializer = getOwnedBySerializer();
          constraintBuffer = serializer.serialize(value as OwnedBy);
          break;
        }
        case OperatorType.PubkeyMatch: {
          const serializer = getPubkeyMatchSerializer();
          constraintBuffer = serializer.serialize(value as PubkeyMatch);
          break;
        }
        default:
          throw new Error('Invalid constraint type');
      }
      // Add the type and constraint size to each constraint.
      const constraintSize = constraintBuffer.length;
      const buffer = new Uint8Array(8 + constraintSize);
      const dataView = new DataView(buffer.buffer);
      dataView.setUint32(0, value.type, true);
      dataView.setUint32(4, constraintSize, true);
      buffer.set(constraintBuffer, 8);
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

      // For some reason the royalty basis points are still on the buffer for the first
      // constraint, so we need to slice it off.
      // However, this stops nested constraints from working, so we need to find a better solution.
      // Unsure why the u64 deserializer doesn't seem to actually remove the basis points from the buffer.
      const constraintData = buffer.slice(16, size + 16);
      let constraint;

      switch (constraintType) {
        case OperatorType.And: {
          const serializer = getAndSerializer();
          constraint = serializer.deserialize(constraintData);
          break;
        }
        case OperatorType.Not: {
          const serializer = getNotSerializer();
          constraint = serializer.deserialize(constraintData);
          break;
        }
        case OperatorType.Or: {
          const serializer = getOrSerializer();
          constraint = serializer.deserialize(constraintData);
          break;
        }
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
