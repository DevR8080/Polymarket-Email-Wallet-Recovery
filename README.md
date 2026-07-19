# Polymarket Email Wallet Recovery

Recover pUSD from Polymarket proxy (smart contract) wallets and withdraw funds directly. This guide only applies to accounts that use Email or Google login.

> **Warning:** This is early, experimental software. Use it at your own risk, and do not use it with large amounts of funds. APIs, commands, and behavior may change without notice. Always verify transaction details before confirming.

## Find Your Private Key

### Quick Start

Log in to Magic.link and retrieve your public address and private key.

```bash
https://reveal.magic.link/
```

Import the private key into a compatible wallet, such as MetaMask.

Next, send a small amount of POL (Polygon) to your Magic wallet's public address to pay for gas fees. Around **$0.50 worth** is sufficient.

## Install

### Homebrew (macOS / Linux)

```bash
brew tap Polymarket/polymarket-cli https://github.com/Polymarket/polymarket-cli
brew install polymarket
```

### Shell Script

```bash
curl -sSL https://raw.githubusercontent.com/Polymarket/polymarket-cli/main/install.sh | sh
```

### Build from Source

```bash
git clone https://github.com/Polymarket/polymarket-cli
cd polymarket-cli
cargo install --path .
```

## Wallet Setup

Import your Magic.link private key to connect the CLI to your Magic wallet.

```bash
polymarket wallet import 0xabc123...
```

## Withdraw Funds

Replace `0xRecipientAddress` with the wallet address that should receive your funds, and replace the amount with the balance of your proxy wallet.

```bash
polymarket transfer --to 0xRecipientAddress --amount 100 --signature-type proxy
```

The CLI will display the transaction ID (TxID). Once the transaction is confirmed, your funds will be transferred to the recipient wallet.

You can then use Uniswap to swap your pUSD for USDC.

## ❤️ Support the Project

If you find this project useful, consider supporting its development. Your donations help improve the project, add new features, and maintain long-term development.

### Cryptocurrency Donations

**EVM Networks (Ethereum, BNB Smart Chain, Polygon, Arbitrum, Base, Optimism, Avalanche C-Chain, etc.)**

USDT / USDC

<img width="250" height="250" alt="qr-code" src="https://github.com/user-attachments/assets/f160a40f-8443-4973-8f7a-4c54abf09921" />

`0x971bf6aBa6Cd0b745b005F2866d4a42cAcAd390C`

> Thank you for supporting the project! ❤️

## License

MIT
