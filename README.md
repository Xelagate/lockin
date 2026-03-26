# LockIn — P2P Escrow on Solana

> Trustless escrow service for crypto ↔ fiat P2P deals, built on Solana.  
> Colosseum Hackathon 2026.

## Overview

LockIn allows two parties to trade cryptocurrency for fiat (bank transfer, cash, etc.) without trusting each other. The smart contract holds tokens in escrow until the seller confirms fiat receipt — eliminating counterparty risk.

## How It Works

```
Seller creates deal → funds escrow → Buyer accepts → pays fiat off-chain
→ Seller confirms receipt → tokens released to Buyer
```

If something goes wrong, either party can open a dispute resolved by the admin key.

## Deal Lifecycle

```
Created → Funded → Accepted → Completed
                            → Cancelled
                 → Disputed → Resolved
```

| Status      | Description                                      |
|-------------|--------------------------------------------------|
| `Created`   | Deal initialized, waiting for seller to deposit  |
| `Funded`    | Seller deposited tokens into escrow PDA          |
| `Accepted`  | Buyer locked in, fiat payment in progress        |
| `Completed` | Seller confirmed fiat received, tokens released  |
| `Cancelled` | Deal cancelled before `Accepted`, tokens refunded|
| `Disputed`  | Dispute opened, waiting for admin resolution     |
| `Resolved`  | Admin resolved the dispute                       |

## Smart Contract Instructions

| Instruction        | Who calls   | Description                                 |
|--------------------|-------------|---------------------------------------------|
| `create_deal`      | Seller      | Creates deal PDA with amount + token mint   |
| `fund_deal`        | Seller      | Deposits SPL tokens into escrow             |
| `accept_deal`      | Buyer       | Locks in as buyer, commits to fiat payment  |
| `confirm_receipt`  | Seller      | Confirms fiat received → releases tokens    |
| `cancel_deal`      | Seller      | Cancels deal, returns tokens to seller      |
| `dispute_resolve`  | Admin       | Resolves dispute in favour of one party     |

## Tech Stack

- **Smart Contract**: Rust + [Anchor 0.32](https://www.anchor-lang.com/)
- **Blockchain**: Solana (devnet / mainnet-beta)
- **Tokens**: SPL Token — USDC / USDT
- **Frontend** *(coming soon)*: Next.js 14 + Tailwind CSS
- **Wallet**: Phantom via `@solana/wallet-adapter`

## Project Structure

```
lockin/
├── programs/lockin/src/lib.rs   # Anchor smart contract
├── tests/                       # Integration tests (TypeScript)
├── app/                         # Next.js frontend (WIP)
├── Anchor.toml
└── Cargo.toml
```

## Getting Started

### Prerequisites

- Rust 1.75+
- Anchor CLI 0.32+
- Solana CLI (configured for devnet)
- Node.js 18+ + Yarn

### Build & Test

```bash
# Install dependencies
yarn install

# Build the program
anchor build

# Run tests
anchor test
```

### Deploy to Devnet

```bash
anchor deploy --provider.cluster devnet
```

## Program ID

```
CsgFGJrJ1feCy1m2yXmD1TEKZNXSfZivY1oEvu7GCW81
```

---

*Built for [Colosseum Hackathon](https://www.colosseum.org/) 2026*
