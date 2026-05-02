# SPL Token Staking Program

An on-chain staking contract built on Solana using the Anchor framework. Users deposit SPL tokens into program-controlled vaults, earn yield over time, and face early withdrawal penalties to incentivize long-term participation.

---

## Features

- **Token staking** — deposit SPL tokens into a PDA-controlled vault
- **Yield accrual** — rewards calculated based on staked amount and duration
- **Lock-up periods** — configurable lock duration per stake entry
- **Early withdrawal penalties** — penalty applied to withdrawals before lock-up expires
- **CPI integration** — uses Cross-Program Invocation to interact with the SPL Token program
- **PDA-based accounts** — stake entries derived from user public keys for trustless custody

---

## Architecture

```
User Wallet
    │
    ▼
[stake instruction]
    │
    ├──► StakeEntry PDA (seeds: ["stake_entry", user])
    │        stores: amount, timestamp, lock_period
    │
    └──► Vault TokenAccount (PDA-controlled)
             holds: staked SPL tokens
```

### Program Accounts

| Account | Type | Description |
|---|---|---|
| `StakeEntry` | PDA | Per-user stake record storing amount and lock metadata |
| `user_token_account` | TokenAccount | User's SPL token account (source) |
| `vault` | TokenAccount | Program-controlled token vault (destination) |

---

## Tech Stack

| Tool | Version |
|---|---|
| Anchor | 0.31.1 |
| Solana platform-tools | v1.52 |
| Rust | 1.89.0-sbpf |
| SPL Token | 7.0.0 |

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/)
- [Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor CLI](https://www.anchor-lang.com/docs/installation) `0.31.1`
- [Node.js](https://nodejs.org/) `>= 18`

### Installation

```bash
git clone https://github.com/YOUR_USERNAME/spl-staking.git
cd spl-staking
yarn install
```

### Build

```bash
# Make sure rustup's cargo shim is first in PATH
export PATH="$HOME/.cargo/bin:$PATH"

anchor build
```

### Local Deployment

```bash
# Terminal 1 — start local validator
solana-test-validator

# Terminal 2 — configure CLI and deploy
solana config set --url localhost
solana airdrop 10
anchor deploy
```

### Run Tests

```bash
anchor test --skip-local-validator
```

---

## Key Concepts

### PDAs (Program Derived Addresses)

Stake entries are stored in PDAs derived from the user's public key, meaning each user has a deterministic on-chain account that only this program can sign for:

```rust
seeds = [b"stake_entry", user.key().as_ref()],
bump,
```

### CPI (Cross-Program Invocation)

Token transfers use CPI calls into the SPL Token program, allowing the staking contract to move tokens on behalf of users without holding private keys:

```rust
token::transfer(cpi_ctx, amount)?;
```

### Lock-up & Penalties

Early withdrawals before the lock period expires incur a configurable penalty, which is either burned or redirected to a rewards pool.

---

## Project Structure

```
spl-staking/
├── programs/
│   └── spl-staking/
│       └── src/
│           └── lib.rs          # Program entrypoint and instructions
├── tests/
│   └── spl-staking.ts          # Integration tests
├── Anchor.toml                 # Anchor workspace config
├── Cargo.toml                  # Workspace manifest
└── package.json
```

---

## Program ID

| Network | Address |
|---|---|
| Localnet | `E4ujhyaQNbyw7epzKuSwEDVP6GmpLanYuPmPaahL3eSn` |

---

## License

MIT
