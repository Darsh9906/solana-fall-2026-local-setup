# Solana Fall 2026 — Local Setup & Devnet Deployment

A hands-on Solana development setup project completed as part of the **Solana Fall 2026** program.

This project covers the fundamentals of working with the Solana CLI and Anchor, including local development, transaction submission, PDA derivation, testing, and deploying an Anchor program to Solana Devnet

## 🚀 What I Built

- Set up a Solana development environment using WSL
- Installed and configured Rust, Solana CLI, Anchor CLI, Node.js, and Yarn
- Configured a Solana wallet and Devnet
- Sent and confirmed a real Devnet transaction
- Created and tested a basic Anchor program
- Derived Program Derived Addresses (PDAs)
- Verified PDA determinism
- Deployed the Anchor program to Solana Devnet
- Verified the deployment using Solana Explorer

## 🛠️ Tech Stack

- Solana
- Anchor Framework
- Rust
- TypeScript
- Node.js
- Yarn
- WSL / Ubuntu
- Solana Devnet

## 📋 Assignment Checkpoints

| Checkpoint | Description | Status |
|---|---|---|
| 0 | Prepare development environment | ✅ |
| 1 | Install required tools | ✅ |
| 2 | Verify installation paths | ✅ |
| 3 | Pin project versions | ✅ |
| 4 | Configure wallet and Devnet | ✅ |
| 5 | Send a Devnet transaction | ✅ |
| 6 | Create and test Anchor project | ✅ |
| 7 | Derive and verify PDAs | ✅ |
| 8 | Deploy program to Devnet | ✅ |
| 9 | Final verification | ✅ |

## 🔑 Program Details

**Program ID**

`HBm4C3VuaCyyEhQBGgPj1Dr4a1FhPzYYj4p6igcJXmcr`

**Network:** Solana Devnet

[View Program on Solana Explorer](https://explorer.solana.com/address/HBm4C3VuaCyyEhQBGgPj1Dr4a1FhPzYYj4p6igcJXmcr?cluster=devnet)

## 🧩 PDA Derivation

### Counter PDA

- Seed: `counter`
- PDA: `GvSgwNKEwHi1UEdGPS8tXEM3sjwSzPVYoaQZcbXWDKaQ`
- Bump: `252`

### Vault PDA

- Seed: `vault`
- PDA: `2vdxwhhmmBT18WEJhYagdfZiGsbD7YUL3ZQ5WLkifQEw`
- Bump: `255`

The `counter` PDA was derived multiple times to verify that the same seed and program ID consistently produce the same address.

This also demonstrates that **deriving a PDA does not automatically create an on-chain account**.

## 💸 Devnet Transaction

A real transaction was submitted and confirmed on Solana Devnet during the setup process.

[View Transaction on Solana Explorer](https://explorer.solana.com/tx/35VnAM6n5SidkEBzGShoPJWDTofNrHwNtdkWS9rPCHUL3HrjPKVzSUhsUiweUuqf6nwKhq6WRqgXDpuCNWFQUThi?cluster=devnet)

## 🧪 Testing

The Anchor program was tested locally using:

```bash
anchor test
