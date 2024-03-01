import { Serializer } from '@metaplex-foundation/umi/serializers';
import { OperatorType } from '../extensions/operatorType';
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
      return constraintBuffer;
    },
    deserialize: (buffer: Uint8Array, offset = 0) => {
      buffer = buffer.slice(offset, buffer.length);
      const dataView = new DataView(
        buffer.buffer,
        buffer.byteOffset,
        buffer.length
      );
      // Manually parse the constraint type. We need the type for our
      // switch statement.
      const constraintType = dataView.getUint32(0, true) as OperatorType;

      let constraint;
      switch (constraintType) {
        case OperatorType.And: {
          const serializer = getAndSerializer();
          [constraint, offset] = serializer.deserialize(buffer);
          break;
        }
        case OperatorType.Not: {
          const serializer = getNotSerializer();
          [constraint, offset] = serializer.deserialize(buffer);
          break;
        }
        case OperatorType.Or: {
          const serializer = getOrSerializer();
          [constraint, offset] = serializer.deserialize(buffer);
          break;
        }
        case OperatorType.OwnedBy: {
          const serializer = getOwnedBySerializer();
          [constraint, offset] = serializer.deserialize(buffer);
          break;
        }
        case OperatorType.PubkeyMatch: {
          const serializer = getPubkeyMatchSerializer();
          [constraint, offset] = serializer.deserialize(buffer);
          break;
        }
        default:
          throw new Error('Invalid constraint type');
      }

      return [constraint, offset];
    },
  };
}
