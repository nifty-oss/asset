import { Delegate, DelegateRole } from './generated';

export const isActive = (delegate: Delegate, role: DelegateRole): boolean =>
  /* eslint-disable-next-line no-bitwise */
  role !== DelegateRole.None && (delegate.roles & (1 << (role - 1))) !== 0;
