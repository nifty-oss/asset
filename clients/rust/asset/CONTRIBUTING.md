# Contributing to the Rust client

This is a quick guide to help you contribute to the Rust client of Nifty Asset.

## Getting started

To build and test the Rust client, you can use `cargo`.

```sh
# Build the client
cargo build

# Test the client (requires building the program first)
cargo test-sbf --sbf-out-dir ../../programs/.bin
```

When something changes in the program(s), make sure to run `pnpm generate` in the root directory, to re-generate the clients accordingly.
