# Contributing to the JavaScript client

This is a quick guide to help you contribute to the JavaScript client of Nifty Bridge.

## Getting started

[Ensure you have pnpm installed](https://pnpm.io/installation) and run the following command to install the client's dependencies.

```sh
pnpm install
```

You can then run the following commands to build, test and lint the client.

```sh
# Build the client.
pnpm build

# Test the client (requires building first).
pnpm build && pnpm test

# Test a specific file or set of files.
pnpm build && pnpm test test/somefile.test.js
pnpm build && pnpm test test/somePattern*

# Lint and/or format the client.
pnpm lint:fix
pnpm format:fix
```

When something changes in the program(s), make sure to run `pnpm generate` in the root directory, to re-generate the clients accordingly.
