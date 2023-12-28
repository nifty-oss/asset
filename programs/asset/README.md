# Asset

A standard for non-fungible assets.

## Building

This will build the program and output a `.so` file in a non-comitted `programs/.bin` directory which is used by the `config/shank.cjs` configuration file to start a new local validator with the latest changes on the program.

```sh
pnpm programs:build
```

## Testing

You may run the following command to build the program and run its Rust tests.

```sh
pnpm programs:test
```
