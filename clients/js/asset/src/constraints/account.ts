import { Serializer, scalarEnum } from '@metaplex-foundation/umi/serializers';

export enum Account {
  Asset,
  Authority,
  Recipient,
}

export type AccountArgs = Account;

export function getAccountSerializer(): Serializer<AccountArgs, Account> {
  return scalarEnum<Account>(Account, {
    description: 'Account',
  }) as Serializer<AccountArgs, Account>;
}
