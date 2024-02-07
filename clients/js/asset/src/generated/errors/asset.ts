/**
 * This code was AUTOGENERATED using the kinobi library.
 * Please DO NOT EDIT THIS FILE, instead use visitors
 * to add features, then rerun kinobi to update it.
 *
 * @see https://github.com/metaplex-foundation/kinobi
 */

import { Program, ProgramError } from '@metaplex-foundation/umi';

type ProgramErrorConstructor = new (
  program: Program,
  cause?: Error
) => ProgramError;
const codeToErrorMap: Map<number, ProgramErrorConstructor> = new Map();
const nameToErrorMap: Map<string, ProgramErrorConstructor> = new Map();

/** AlreadyInitialized: Asset already initialized */
export class AlreadyInitializedError extends ProgramError {
  override readonly name: string = 'AlreadyInitialized';

  readonly code: number = 0x0; // 0

  constructor(program: Program, cause?: Error) {
    super('Asset already initialized', program, cause);
  }
}
codeToErrorMap.set(0x0, AlreadyInitializedError);
nameToErrorMap.set('AlreadyInitialized', AlreadyInitializedError);

/** InvalidAccountLength: Invalid account length */
export class InvalidAccountLengthError extends ProgramError {
  override readonly name: string = 'InvalidAccountLength';

  readonly code: number = 0x1; // 1

  constructor(program: Program, cause?: Error) {
    super('Invalid account length', program, cause);
  }
}
codeToErrorMap.set(0x1, InvalidAccountLengthError);
nameToErrorMap.set('InvalidAccountLength', InvalidAccountLengthError);

/** IncompleteExtensionData: Incomplete extension data */
export class IncompleteExtensionDataError extends ProgramError {
  override readonly name: string = 'IncompleteExtensionData';

  readonly code: number = 0x2; // 2

  constructor(program: Program, cause?: Error) {
    super('Incomplete extension data', program, cause);
  }
}
codeToErrorMap.set(0x2, IncompleteExtensionDataError);
nameToErrorMap.set('IncompleteExtensionData', IncompleteExtensionDataError);

/** Uninitialized: Uninitialized account */
export class UninitializedError extends ProgramError {
  override readonly name: string = 'Uninitialized';

  readonly code: number = 0x3; // 3

  constructor(program: Program, cause?: Error) {
    super('Uninitialized account', program, cause);
  }
}
codeToErrorMap.set(0x3, UninitializedError);
nameToErrorMap.set('Uninitialized', UninitializedError);

/** ExtensionNotFound: Extension not found */
export class ExtensionNotFoundError extends ProgramError {
  override readonly name: string = 'ExtensionNotFound';

  readonly code: number = 0x4; // 4

  constructor(program: Program, cause?: Error) {
    super('Extension not found', program, cause);
  }
}
codeToErrorMap.set(0x4, ExtensionNotFoundError);
nameToErrorMap.set('ExtensionNotFound', ExtensionNotFoundError);

/** InvalidAlignment: Invalid alignment */
export class InvalidAlignmentError extends ProgramError {
  override readonly name: string = 'InvalidAlignment';

  readonly code: number = 0x5; // 5

  constructor(program: Program, cause?: Error) {
    super('Invalid alignment', program, cause);
  }
}
codeToErrorMap.set(0x5, InvalidAlignmentError);
nameToErrorMap.set('InvalidAlignment', InvalidAlignmentError);

/** InvalidBurnAuthority: Invalid holder or burn delegate */
export class InvalidBurnAuthorityError extends ProgramError {
  override readonly name: string = 'InvalidBurnAuthority';

  readonly code: number = 0x6; // 6

  constructor(program: Program, cause?: Error) {
    super('Invalid holder or burn delegate', program, cause);
  }
}
codeToErrorMap.set(0x6, InvalidBurnAuthorityError);
nameToErrorMap.set('InvalidBurnAuthority', InvalidBurnAuthorityError);

/** InvalidTransferAuthority: Invalid holder or transfer delegate */
export class InvalidTransferAuthorityError extends ProgramError {
  override readonly name: string = 'InvalidTransferAuthority';

  readonly code: number = 0x7; // 7

  constructor(program: Program, cause?: Error) {
    super('Invalid holder or transfer delegate', program, cause);
  }
}
codeToErrorMap.set(0x7, InvalidTransferAuthorityError);
nameToErrorMap.set('InvalidTransferAuthority', InvalidTransferAuthorityError);

/** DelegateNotFound: Delegate not found */
export class DelegateNotFoundError extends ProgramError {
  override readonly name: string = 'DelegateNotFound';

  readonly code: number = 0x8; // 8

  constructor(program: Program, cause?: Error) {
    super('Delegate not found', program, cause);
  }
}
codeToErrorMap.set(0x8, DelegateNotFoundError);
nameToErrorMap.set('DelegateNotFound', DelegateNotFoundError);

/** DelegateRoleNotActive: Delegate role not active */
export class DelegateRoleNotActiveError extends ProgramError {
  override readonly name: string = 'DelegateRoleNotActive';

  readonly code: number = 0x9; // 9

  constructor(program: Program, cause?: Error) {
    super('Delegate role not active', program, cause);
  }
}
codeToErrorMap.set(0x9, DelegateRoleNotActiveError);
nameToErrorMap.set('DelegateRoleNotActive', DelegateRoleNotActiveError);

/** InvalidDelegate: Invalid delegate */
export class InvalidDelegateError extends ProgramError {
  override readonly name: string = 'InvalidDelegate';

  readonly code: number = 0xa; // 10

  constructor(program: Program, cause?: Error) {
    super('Invalid delegate', program, cause);
  }
}
codeToErrorMap.set(0xa, InvalidDelegateError);
nameToErrorMap.set('InvalidDelegate', InvalidDelegateError);

/** InvalidHolder: Invalid holder */
export class InvalidHolderError extends ProgramError {
  override readonly name: string = 'InvalidHolder';

  readonly code: number = 0xb; // 11

  constructor(program: Program, cause?: Error) {
    super('Invalid holder', program, cause);
  }
}
codeToErrorMap.set(0xb, InvalidHolderError);
nameToErrorMap.set('InvalidHolder', InvalidHolderError);

/** LockedAsset: Asset is locked */
export class LockedAssetError extends ProgramError {
  override readonly name: string = 'LockedAsset';

  readonly code: number = 0xc; // 12

  constructor(program: Program, cause?: Error) {
    super('Asset is locked', program, cause);
  }
}
codeToErrorMap.set(0xc, LockedAssetError);
nameToErrorMap.set('LockedAsset', LockedAssetError);

/** InvalidAuthority: Invalid authority */
export class InvalidAuthorityError extends ProgramError {
  override readonly name: string = 'InvalidAuthority';

  readonly code: number = 0xd; // 13

  constructor(program: Program, cause?: Error) {
    super('Invalid authority', program, cause);
  }
}
codeToErrorMap.set(0xd, InvalidAuthorityError);
nameToErrorMap.set('InvalidAuthority', InvalidAuthorityError);

/** ImmutableAsset: Immutable asset */
export class ImmutableAssetError extends ProgramError {
  override readonly name: string = 'ImmutableAsset';

  readonly code: number = 0xe; // 14

  constructor(program: Program, cause?: Error) {
    super('Immutable asset', program, cause);
  }
}
codeToErrorMap.set(0xe, ImmutableAssetError);
nameToErrorMap.set('ImmutableAsset', ImmutableAssetError);

/** CannotTransferSoulbound: Soulbound assets are non-transferable */
export class CannotTransferSoulboundError extends ProgramError {
  override readonly name: string = 'CannotTransferSoulbound';

  readonly code: number = 0xf; // 15

  constructor(program: Program, cause?: Error) {
    super('Soulbound assets are non-transferable', program, cause);
  }
}
codeToErrorMap.set(0xf, CannotTransferSoulboundError);
nameToErrorMap.set('CannotTransferSoulbound', CannotTransferSoulboundError);

/** ExtensionDataInvalid: Extension data invalid */
export class ExtensionDataInvalidError extends ProgramError {
  override readonly name: string = 'ExtensionDataInvalid';

  readonly code: number = 0x10; // 16

  constructor(program: Program, cause?: Error) {
    super('Extension data invalid', program, cause);
  }
}
codeToErrorMap.set(0x10, ExtensionDataInvalidError);
nameToErrorMap.set('ExtensionDataInvalid', ExtensionDataInvalidError);

/** InvalidGroup: Invalid group */
export class InvalidGroupError extends ProgramError {
  override readonly name: string = 'InvalidGroup';

  readonly code: number = 0x11; // 17

  constructor(program: Program, cause?: Error) {
    super('Invalid group', program, cause);
  }
}
codeToErrorMap.set(0x11, InvalidGroupError);
nameToErrorMap.set('InvalidGroup', InvalidGroupError);

/**
 * Attempts to resolve a custom program error from the provided error code.
 * @category Errors
 */
export function getAssetErrorFromCode(
  code: number,
  program: Program,
  cause?: Error
): ProgramError | null {
  const constructor = codeToErrorMap.get(code);
  return constructor ? new constructor(program, cause) : null;
}

/**
 * Attempts to resolve a custom program error from the provided error name, i.e. 'Unauthorized'.
 * @category Errors
 */
export function getAssetErrorFromName(
  name: string,
  program: Program,
  cause?: Error
): ProgramError | null {
  const constructor = nameToErrorMap.get(name);
  return constructor ? new constructor(program, cause) : null;
}
