import {
  Serializer,
  scalarEnum,
  u64,
} from '@metaplex-foundation/umi/serializers';

export enum Account {
  Asset,
  Authority,
  Recipient,
}

export type AccountArgs = Account;

export function getAccountSerializer(): Serializer<AccountArgs, Account> {
  return scalarEnum<Account>(Account, {
    size: u64(),
  }) as Serializer<AccountArgs, Account>;
}
