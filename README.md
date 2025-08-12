# Solana DBotX Helius Bot

Prototype trading bot that fetches 1m kline data from the DBotX API and
generates "volume dump" signals. Orders can be executed in a dry run mode or
sent to on-chain executors (not yet implemented).

## Running

```bash
cargo run --release            # default DRY mode
EXEC_MODE=LIVE cargo run --release
```

Configuration is loaded from environment variables. You can copy `.env.example`
to `.env` and fill in your secrets:

```bash
cp .env.example .env
```

The key variables are:

```bash
DBOTX_API_KEY=...
DBOTX_KLINE_URL="https://api-data-v1.dbotx.com/kline/chart?chain={chain}&pair={pair}&interval={interval}&end={end}"
DBOTX_CHAIN=solana
DBOTX_PAIR=PAIR_ADDRESS
RPC_URL=https://mainnet.helius-rpc.com/?api-key=...
PUBLIC_KEY=...
SECURE_CODE=...
BASE=TOKEN
QUOTE=SOL
ORDER_SIZE_SOL=0.5
```

For a detailed Chinese deployment guide, see [`docs/deploy.zh.md`](docs/deploy.zh.md).

This codebase only implements a subset of the intended functionality and is
meant as a foundation for further development.

