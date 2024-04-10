# Proxy

A novel pattern to define extendable access control to program accounts.

## Building

This will build the program and output a `.so` file in a non-comitted `target/deploy` directory which is used by the `config/shank.cjs` configuration file to start a new local validator with the latest changes on the program.

```sh
cargo build-sbf
```

## Testing

You may run the following command to build the program and run its Rust tests.

```sh
cargo test-sbf
```
