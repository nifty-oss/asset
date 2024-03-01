<h1 align="center">
  Nifty Asset
</h1>
<p align="center">
  <img width="400" alt="Nifty Asset" src="https://github.com/nifty-oss/asset/assets/729235/90513ea4-ce87-41b5-88a4-008991fc73a7" />
</p>
<p align="center">
  A lightweight standard for non-fungible assets.
</p>

<p align="center">
  <a href="https://github.com/nifty-oss/asset/actions/workflows/main.yml"><img src="https://img.shields.io/github/actions/workflow/status/nifty-oss/asset/main.yml?logo=GitHub" /></a>
  <!--
  <a href="https://explorer.solana.com/address/AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73"><img src="https://img.shields.io/badge/dynamic/json?url=https%3A%2F%2Fraw.githubusercontent.com%2Fnifty-oss%2Fasset%2Fmain%2Fidls%2Fasset_program.json&query=%24.version&label=program%20version&logo=data:image/svg%2bxml;base64,PHN2ZyB3aWR0aD0iMzEzIiBoZWlnaHQ9IjI4MSIgdmlld0JveD0iMCAwIDMxMyAyODEiIGZpbGw9Im5vbmUiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CjxnIGNsaXAtcGF0aD0idXJsKCNjbGlwMF80NzZfMjQzMCkiPgo8cGF0aCBkPSJNMzExLjMxOCAyMjEuMDU3TDI1OS42NiAyNzYuNTU4QzI1OC41MzcgMjc3Ljc2NCAyNTcuMTc4IDI3OC43MjUgMjU1LjY2OSAyNzkuMzgyQzI1NC4xNTkgMjgwLjAzOSAyNTIuNTMgMjgwLjM3OCAyNTAuODg0IDI4MC4zNzdINS45OTcxOUM0LjgyODcgMjgwLjM3NyAzLjY4NTY4IDI4MC4wMzUgMi43MDg1NSAyNzkuMzkzQzEuNzMxNDMgMjc4Ljc1MSAwLjk2Mjc3MSAyNzcuODM3IDAuNDk3MDIgMjc2Ljc2NEMwLjAzMTI2OTEgMjc1LjY5IC0wLjExMTI4NiAyNzQuNTA0IDAuMDg2ODcxMiAyNzMuMzVDMC4yODUwMjggMjcyLjE5NiAwLjgxNTI2NSAyNzEuMTI2IDEuNjEyNDMgMjcwLjI3TDUzLjMwOTkgMjE0Ljc2OUM1NC40Mjk5IDIxMy41NjYgNTUuNzg0MyAyMTIuNjA3IDU3LjI4OTMgMjExLjk1QzU4Ljc5NDMgMjExLjI5MyA2MC40MTc4IDIxMC45NTMgNjIuMDU5NSAyMTAuOTVIMzA2LjkzM0MzMDguMTAxIDIxMC45NSAzMDkuMjQ0IDIxMS4yOTIgMzEwLjIyMSAyMTEuOTM0QzMxMS4xOTkgMjEyLjU3NiAzMTEuOTY3IDIxMy40OSAzMTIuNDMzIDIxNC41NjRDMzEyLjg5OSAyMTUuNjM3IDMxMy4wNDEgMjE2LjgyNCAzMTIuODQzIDIxNy45NzdDMzEyLjY0NSAyMTkuMTMxIDMxMi4xMTUgMjIwLjIwMSAzMTEuMzE4IDIyMS4wNTdaTTI1OS42NiAxMDkuMjk0QzI1OC41MzcgMTA4LjA4OCAyNTcuMTc4IDEwNy4xMjcgMjU1LjY2OSAxMDYuNDdDMjU0LjE1OSAxMDUuODEzIDI1Mi41MyAxMDUuNDc0IDI1MC44ODQgMTA1LjQ3NUg1Ljk5NzE5QzQuODI4NyAxMDUuNDc1IDMuNjg1NjggMTA1LjgxNyAyLjcwODU1IDEwNi40NTlDMS43MzE0MyAxMDcuMTAxIDAuOTYyNzcxIDEwOC4wMTUgMC40OTcwMiAxMDkuMDg4QzAuMDMxMjY5MSAxMTAuMTYyIC0wLjExMTI4NiAxMTEuMzQ4IDAuMDg2ODcxMiAxMTIuNTAyQzAuMjg1MDI4IDExMy42NTYgMC44MTUyNjUgMTE0LjcyNiAxLjYxMjQzIDExNS41ODJMNTMuMzA5OSAxNzEuMDgzQzU0LjQyOTkgMTcyLjI4NiA1NS43ODQzIDE3My4yNDUgNTcuMjg5MyAxNzMuOTAyQzU4Ljc5NDMgMTc0LjU1OSA2MC40MTc4IDE3NC44OTkgNjIuMDU5NSAxNzQuOTAySDMwNi45MzNDMzA4LjEwMSAxNzQuOTAyIDMwOS4yNDQgMTc0LjU2IDMxMC4yMjEgMTczLjkxOEMzMTEuMTk5IDE3My4yNzYgMzExLjk2NyAxNzIuMzYyIDMxMi40MzMgMTcxLjI4OEMzMTIuODk5IDE3MC4yMTUgMzEzLjA0MSAxNjkuMDI4IDMxMi44NDMgMTY3Ljg3NUMzMTIuNjQ1IDE2Ni43MjEgMzEyLjExNSAxNjUuNjUxIDMxMS4zMTggMTY0Ljc5NUwyNTkuNjYgMTA5LjI5NFpNNS45OTcxOSA2OS40MjY3SDI1MC44ODRDMjUyLjUzIDY5LjQyNzUgMjU0LjE1OSA2OS4wODkgMjU1LjY2OSA2OC40MzJDMjU3LjE3OCA2Ny43NzUxIDI1OC41MzcgNjYuODEzOSAyNTkuNjYgNjUuNjA4MkwzMTEuMzE4IDEwLjEwNjlDMzEyLjExNSA5LjI1MTA3IDMxMi42NDUgOC4xODA1NiAzMTIuODQzIDcuMDI2OTVDMzEzLjA0MSA1Ljg3MzM0IDMxMi44OTkgNC42ODY4NiAzMTIuNDMzIDMuNjEzM0MzMTEuOTY3IDIuNTM5NzQgMzExLjE5OSAxLjYyNTg2IDMxMC4yMjEgMC45ODM5NDFDMzA5LjI0NCAwLjM0MjAyNiAzMDguMTAxIDMuOTUzMTRlLTA1IDMwNi45MzMgMEw2Mi4wNTk1IDBDNjAuNDE3OCAwLjAwMjc5ODY2IDU4Ljc5NDMgMC4zNDMxNCA1Ny4yODkzIDAuOTk5OTUzQzU1Ljc4NDMgMS42NTY3NyA1NC40Mjk5IDIuNjE2MDcgNTMuMzA5OSAzLjgxODQ3TDEuNjI1NzYgNTkuMzE5N0MwLjgyOTM2MSA2MC4xNzQ4IDAuMjk5MzU5IDYxLjI0NCAwLjEwMDc1MiA2Mi4zOTY0Qy0wLjA5Nzg1MzkgNjMuNTQ4OCAwLjA0MzU2OTggNjQuNzM0MiAwLjUwNzY3OSA2NS44MDczQzAuOTcxNzg5IDY2Ljg4MDMgMS43Mzg0MSA2Ny43OTQzIDIuNzEzNTIgNjguNDM3MkMzLjY4ODYzIDY5LjA4MDIgNC44Mjk4NCA2OS40MjQgNS45OTcxOSA2OS40MjY3WiIgZmlsbD0idXJsKCNwYWludDBfbGluZWFyXzQ3Nl8yNDMwKSIvPgo8L2c+CjxkZWZzPgo8bGluZWFyR3JhZGllbnQgaWQ9InBhaW50MF9saW5lYXJfNDc2XzI0MzAiIHgxPSIyNi40MTUiIHkxPSIyODcuMDU5IiB4Mj0iMjgzLjczNSIgeTI9Ii0yLjQ5NTc0IiBncmFkaWVudFVuaXRzPSJ1c2VyU3BhY2VPblVzZSI+CjxzdG9wIG9mZnNldD0iMC4wOCIgc3RvcC1jb2xvcj0iIzk5NDVGRiIvPgo8c3RvcCBvZmZzZXQ9IjAuMyIgc3RvcC1jb2xvcj0iIzg3NTJGMyIvPgo8c3RvcCBvZmZzZXQ9IjAuNSIgc3RvcC1jb2xvcj0iIzU0OTdENSIvPgo8c3RvcCBvZmZzZXQ9IjAuNiIgc3RvcC1jb2xvcj0iIzQzQjRDQSIvPgo8c3RvcCBvZmZzZXQ9IjAuNzIiIHN0b3AtY29sb3I9IiMyOEUwQjkiLz4KPHN0b3Agb2Zmc2V0PSIwLjk3IiBzdG9wLWNvbG9yPSIjMTlGQjlCIi8+CjwvbGluZWFyR3JhZGllbnQ+CjxjbGlwUGF0aCBpZD0iY2xpcDBfNDc2XzI0MzAiPgo8cmVjdCB3aWR0aD0iMzEyLjkzIiBoZWlnaHQ9IjI4MC4zNzciIGZpbGw9IndoaXRlIi8+CjwvY2xpcFBhdGg+CjwvZGVmcz4KPC9zdmc+Cg==&color=9945FF" /></a>
  <a href="https://www.npmjs.com/package/@nifty-oss/asset"><img src="https://img.shields.io/npm/v/%40nifty-oss%2Fasset?logo=npm&color=377CC0" /></a>
  <a href="https://crates.io/crates/nifty-asset"><img src="https://img.shields.io/crates/v/nifty-asset?logo=rust" /></a>
  -->
</p>

## Contents

- [Features](#features)
- [Overview](#overview)
- [Packages](#packages)
- [Building](#building)
- [Testing](#testing)
- [Programs](#programs)
- [Clients](#clients)
- [CLI](#cli)
- [License](#license)

> [!WARNING]
> `nifty asset` has not been formally audited – use in production at your own risk.

## Features

The main goal of `nifty asset` is to provide an open-source, lightweight and composable non-fungible standard on Solana. This is achieved by:
- Using a **single** account to represent a digital asset.
- Storing as much or as little data on-chain using optional extensions.
- Efficient zero-copy de-/serialization to minimize compute units utilization.
- Full-featured standard, including royalty enforcement, delegates, lock/unlock, inscriptions and groups (collections).
- Rust and JavaScript client SDKs.

## Overview

Majority of NFT standards on Solana are built on top of SPL Token, which is mainly a fungible token program. A non-fungible is created by adding restrictions to fungible mints – i.e., for a mint to be considered "non-fungible", it must have supply `1`, decimals `0` and no mint authority. As a consequence, NFT standards carry "baggage" from the requirements of fungibles to represent non-fungibles. For example, while non-fungibles have a supply of `1` by definition, there is still a requirement for separate mint and token accounts. Requiring a separate mint and token accounts is wasteful and adds complexity (e.g., more accounts validation) to a standard. In most cases, the only information required from a non-fungible token account is the holder (owner) address, given that the amount is always expected to be `1`. At the same time, a token account takes `165` bytes of storage. Additionally, SPL Token - even including Token Extensions (a.k.a., SPL Token 2022) - does not enforce constraints to define different token standards.

Since different token standards will usually be defined in separate programs, the end result is a bloated (multiple accounts) standard – e.g., `mint`, `token` and (potentially) `metadata` accounts, in addition to SPL Token and Metadata programs. For most operations, all `5` accounts would be needed to interact with a non-fungible – and there are standards that the number of accounts is even higher.

`nifty asset` takes a different approach and re-imagines how non-fungibles are represented on Solana. In essense, a non-fungible asset is a unique *slab* of bytes on the blockchain identified by an address and it has an specific owner. When you transfer the "ownership" of this slab of bytes, you are changing the owner information of it. The contents of a non-fungible is as flexible as possible, given that they are "just" a slab of bytes – all data can be on-chain, or it can have "pointers" to external resources.

## Packages

The packages below can be used to interact with `nifty asset` program.

### JavaScript
```bash
npm install @nifty-oss/asset
```

### Rust
```bash
cargo add nifty-asset
```

## Building

To build the programs from the root directory of the repository:
```bash
pnpm install
```
to install the required libraries, then:
```bash
pnpm programs:build
```

This will create program binaries at `<ROOT>/programs/.bin`.

## Testing

The repository contains both Rust BPF and TypeScript test. These can be run from the root directory of the repository:
```bash
pnpm clients:rust:test
```
for Rust BPF tests, and:
```bash
pnpm clients:js:test
```
for TypeScript tests.

## Programs

This project contains the following programs:

- [Asset](./programs/asset/README.md) `AssetGtQBTSgm5s91d1RAQod5JmaZiJDxqsgtqrZud73`

- [Bridge](./programs/bridge/README.md) `BridgezKrNugsZwTcyAMYba643Z93RzC2yN1Y24LwAkm`

You will need a Rust version compatible with BPF to compile the program, currently we recommend using Rust 1.68.0.

## Clients

Each program in the project contains autogenerated Javascript and Rust clients:

Asset:
- [JavaScript](./clients/js/asset/README.md)
- [Rust](./clients/rust/asset/README.md)

Bridge:
- [JavaScript](./clients/js/bridge/README.md)
- [Rust](./clients/rust/bridge/README.md)

## CLI

The project includes a Rust-based command-line interface:

- [`nifty`](./clients/cli/README.md)

## Contributing

Check out the [Contributing Guide](./CONTRIBUTING.md) the learn more about how to contribute to this project.

## License

Copyright (c) 2024 nifty-oss maintainers

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
