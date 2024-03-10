# Nifty Asset Types

Types for Nifty Asset [program](https://github.com/nifty-oss/asset).

The types defined in this crate are used to represent assets on-chain.

- `constraints` - these are types to define constraints when manipulating assets. They
can be used to restrict the accounts that can hold, receive or send assets.

- `extensions` - these are types that provide additional data that can be attached to an asset.They can be used to store more information about an asset on-chain or extends their behaviour.

- `state` - these are the types represeting the account that store the state of an asset on-chain.

This crate is usually used as part of the [Nifty Asset SDK](https://crates.io/crates/nifty-asset).