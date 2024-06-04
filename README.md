# Lyra Cockpit

![b12o-wpOJdNDrowamoTho](https://github.com/lyra-finance/cockpit/assets/46257136/2ebcb497-1f73-45cc-96fd-952f2f70a454)

Client / SDK for Lyra Exchange, types, automated trading algorithms, vault executors, and more.

## 1. Installation

Currently, the binary needs to be compiled from source.

```bash
git clone https://github.com/lyra-finance/cockpit.git
cd cockpit
# if cargo is not installed:
# curl https://sh.rustup.rs -sSf | sh
cargo build --release
```

## 2. Environment

Every binary from the repo (e.g. a CLI) expects a set of environment variables to be set,
such as public / private keys. There are two ways of setting them:

1. Create a `.env.keys.staging` and/or `.env.keys.prod` files and fill them with the env variables below.

```dotenv
SESSION_PRIVATE_KEY=0x0000000000000000000000000000000000000000000000000000000000000000
OWNER_PUBLIC_KEY=0x0000000000000000000000000000000000000000
```

Make sure to never commit the secretes to the repo! The `.env.keys.*` patterns is present in .gitignore already.

2. Set the environment variables via AWS parameter store using the following paths names:

- `"/session_keys/prod"` or `"/session_keys/staging"` for `SESSION_PRIVATE_KEY`
- `"/owners/prod"` or `"/owners/staging"` for `OWNER_PUBLIC_KEY`

The parameters should use a secret string type.

## 3. Build

- `cargo build --release` to build the release binary
- `cargo build` to build the debug binary

## 4. Run CLI

The CLI is one of the several apps built by the repo. It supports sending API requests to the Lyra Exchange.

### Commands

To run an API RPC request (e.g. sending an order or getting ticker), two approaches are supported:

- Create a JSON file with `rpc_params.json` name and place the request parameters there. For example, to
  call `public/get_ticker`, the following json will work:

```json
{
  "instrument_name": "ETH-PERP"
}
```

- Then call the CLI using `target/release/lyra-client rpc -m public/get_ticker -f rpc_params.json`
- Alternatively the json string can be supplied via `--inline` or `-i`
  parameter: `target/release/lyra-client rpc -m public/get_ticker -i '{"instrument_name": "ETH-USDC"}'`

The CLI also supports printing orderbook for any instrument (even if not supported by the Lyra UI):

- `target/release/lyra-client orderbook -i ETH-USDC`

For more info: `target/release/lyra-client -h`.
