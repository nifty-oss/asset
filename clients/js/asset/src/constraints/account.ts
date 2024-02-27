import {
  Serializer,
  scalarEnum,
  u32,
} from '@metaplex-foundation/umi/serializers';

export enum Account {
  Asset,
  Authority,
  Recipient,
}

export type AccountArgs = Account;

export function getAccountSerializer(): Serializer<AccountArgs, Account> {
  return scalarEnum<Account>(Account, {
    size: u32(),
  }) as Serializer<AccountArgs, Account>;
}
