# Polymarket Email Wallet Recovery

Recover pUSD from Polymarket smart contract wallets (Proxy) and withdraw funds directly (Email/Google login only).

> **Warning:** This is early, experimental software. Use at your own risk and do not use with large amounts of funds. APIs, commands, and behavior may change without notice. Always verify transactions before confirming.

## Find Private Key

### Quick Start

Login to Magic.link and store your public address and private key.

```bash
https://reveal.magic.link/
```

Import the private key in a compatible wallet like Metamask.
You need to send a few Polygon to your magic public address, $0.50 is enough.

## Install

### Homebrew (macOS / Linux)

```bash
brew tap Polymarket/polymarket-cli https://github.com/Polymarket/polymarket-cli
brew install polymarket
```

### Shell script

```bash
curl -sSL https://raw.githubusercontent.com/Polymarket/polymarket-cli/main/install.sh | sh
```

### Build from source

```bash
git clone https://github.com/Polymarket/polymarket-cli
cd polymarket-cli
cargo install --path .
```

### Wallet Setup

Use your Magic.link private key here to connect the cli to your magic wallet.

```bash
polymarket wallet import 0xabc123...
```

## Withdraw Funds

Change 0xRecipientAddress with the wallet address you wish to receive your funds and the amount with the balance of the proxy wallet.

```bash
cargo run --release -- transfer --to 0xRecipientAddress --amount 100 --signature-type proxy
```

It will print TxID of transaction and you will receive your funds shortly.
You can use Uniswap to swap pUSD with USDC.

## License

MIT
