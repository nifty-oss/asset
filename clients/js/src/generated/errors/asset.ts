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
