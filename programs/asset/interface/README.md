# <img width="325" alt="nifty-asset-types" src="https://github.com/nifty-oss/asset/assets/729235/cfa1923e-73a6-49e5-89ca-0cbe54cdb591"/>

Interface for Nifty Asset [program](https://github.com/nifty-oss/asset).

The interface defined in this crate should be used to implement proxy programs extending Nifty Asset behaviour. It follows the [Proxy Pattern](https://nifty-oss.org/blog/proxy-pattern) to provide a program interface for developers to build on top and fully customise Nifty Asset.

## Getting started

From your project folder:

```bash
cargo add nifty-asset-interface
```

## Structure

The SDK is divided into several modules:

- `instructions`: structs to facilitate the creation of instructions on-chain
- `types`: structs representing types used by the program
