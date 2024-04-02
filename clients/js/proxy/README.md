# Nifty Proxy

A Umi-compatible JavaScript library for Nifty Proxy [program](https://github.com/nifty-oss/asset).

## Getting started

1. First, if you're not already using Umi, [follow these instructions to install the Umi framework](https://github.com/metaplex-foundation/umi/blob/main/docs/installation.md).

2. Next, install this library using the package manager of your choice.
   ```sh
   npm install @nifty-oss/proxy
   ```
2. Finally, register the library with your Umi instance like so.
   ```ts
   import { niftyProxy } from '@nifty-oss/proxy';
   umi.use(niftyProxy());
   ```
