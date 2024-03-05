# Nifty CLI

A CLI for interacting with the Nifty asset program.

## Installation

From source:

```bash
cargo install --path .
```

## Commands

### Burn

Burns an asset, closing the account reclaiming all rent.

```
Usage: nifty burn [OPTIONS] <ASSET> [RECIPIENT]

Arguments:
  <ASSET>      The asset to burn
  [RECIPIENT]  The recipient to receive reclaimed rent. Defaults to the signer

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

Examples: 

No recipient specified, so reclaimed rent goes to the signing keypair:

```bash
nifty burn 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
```

Recipient specified and receives reclaimed rent:

```bash
nifty burn 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP 9Z3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
```

### Create

Creates a new asset.

```
Usage: nifty create [OPTIONS] --name <NAME>

Options:
  -k, --keypair-path <KEYPAIR_PATH>
          Path to the keypair file
  -n, --name <NAME>
          The name of the asset
  -a, --asset-keypair-path <ASSET_KEYPAIR_PATH>
          Path to the mint keypair file
  -r, --rpc-url <RPC_URL>
          RPC URL for the Solana cluster
      --immutable
          Create the asset as immutable
  -o, --owner <OWNER>
          Owner of the created asset, defaults to authority pubkey
  -h, --help
          Print help
  ```

Examples:

Create a mutable asset:

```bash
nifty create --name "My Asset"
```

Create an immutable asset:

```bash
nifty create --name "My Immutable Asset" --immutable
```

Create an asset with a specific owner:

```bash
nifty create --name "My Asset" --owner 9Z3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
```

Create an asset from an existing keypair file:

```bash
nifty create --name "My Asset" --asset-keypair-path /path/to/asset-keypair.json
```

### Decode

Decodes an asset into a human readable format.

```
Usage: nifty decode [OPTIONS] <ASSET>

Arguments:
  <ASSET>

Options:
  -f, --field <FIELD>                The field to decode. If not specified, the entire asset will be decoded
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

Example:

```bash
nifty decode 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
```

Decode a specific field:

```bash
nifty decode 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP --field state
```

prints only the state information of the asset.

### Delegate

Set a delegate on an asset with specific roles

```
Usage: nifty delegate [OPTIONS] <ASSET> <DELEGATE>

Arguments:
  <ASSET>     The asset to delegate
  <DELEGATE>  The address to delegate to

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -R, --role <ROLE>                  The role for the delegate to have: "burn", "lock", "transfer". Specify each one separately: --role burn --role lock --role transfer
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

Example:

```bash
nifty delegate 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP 9Z3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP --role burn --role lock
```

### Lock

Lock an asset, preventing any actions to be performed on it.

```
Usage: nifty lock [OPTIONS] <ASSET> [DELEGATE_KEYPAIR_PATH]

Arguments:
  <ASSET>                  The asset to lock
  [DELEGATE_KEYPAIR_PATH]  Path to the delegate keypair file. Defaults to the signer

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

Example:

```bash
nifty lock 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
```

The keypair used must be the delegate of the asset.


### Unlock

Unlock an asset, allowing actions to be performed on it.

```
Usage: nifty unlock [OPTIONS] <ASSET> [DELEGATE_KEYPAIR_PATH]

Arguments:
  <ASSET>                  The asset to unlock
  [DELEGATE_KEYPAIR_PATH]  Path to the delegate keypair file. Defaults to the signer

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
```

Example:

```bash
nifty unlock 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
```

The keypair used must be the delegate of the asset.

### Transfer

Transfers an asset to a new owner.

```
Usage: nifty transfer [OPTIONS] <ASSET> <RECIPIENT>

Arguments:
  <ASSET>      The asset to transfer
  <RECIPIENT>  The recipient of the asset

Options:
  -k, --keypair-path <KEYPAIR_PATH>  Path to the keypair file
  -r, --rpc-url <RPC_URL>            RPC URL for the Solana cluster
  -h, --help                         Print help
  ```

Example:

  ```bash
  nifty transfer 92D3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP 9Z3tDoqtREj3Exkr5ws9UPawG3yhaEwjSP4J5GumuRP
  ```
