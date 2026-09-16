# 🔗 Gono Protocol

> **Provenance infrastructure for humans and AI**

<p align="center">
  <img src="https://img.shields.io/badge/Polkadot%20SDK-2606.0.0-E6007A?style=for-the-badge&logo=polkadot" alt="Polkadot SDK"/>
  <img src="https://img.shields.io/badge/Substrate-FRAME%20v48-blue?style=for-the-badge&logo=polkadot" alt="Substrate FRAME"/>
  <img src="https://img.shields.io/badge/Rust-2021-orange?style=for-the-badge&logo=rust" alt="Rust"/>
  <img src="https://img.shields.io/badge/Tests-85%20Passing-brightgreen?style=for-the-badge" alt="Tests"/>
  <img src="https://img.shields.io/github/actions/workflow/status/Meherajs/gono-protocol-website/ci.yml?branch=main&style=for-the-badge&label=CI" alt="CI Status"/>
  <img src="https://img.shields.io/badge/Next.js-16-black?style=for-the-badge&logo=next.js" alt="Next.js"/>
  <img src="https://img.shields.io/badge/React-19-61dafb?style=for-the-badge&logo=react" alt="React"/>
  <img src="https://img.shields.io/badge/Tailwind-4.0-38bdf8?style=for-the-badge&logo=tailwindcss" alt="Tailwind"/>
  <img src="https://img.shields.io/badge/Actix--web-4.0-000000?style=for-the-badge" alt="Actix"/>
  <img src="https://img.shields.io/badge/Vercel-Deployed-000?style=for-the-badge&logo=vercel" alt="Vercel"/>
</p>

---

## 🏢 About Gono Protocol

Gono Protocol is a modular blockchain infrastructure built on Substrate as a Polkadot Parachain. It provides a universal, content-addressable rail for verifiable media, digital assets, and autonomous AI commerce using a pluggable pallet architecture and a Three-Tier Sovereign Governance model:

- **ERC-7053 Media Receipts & C2PA Provenance** (`pallet-gono-store`) — ✅ Complete (14 tests)
- **SANUB Credibility Scoring & Reputation Engine** (`pallet-gono-verify`) — ✅ Complete (13 tests)
- **HTTP 402 AI-Native Micropayments** (`pallet-gono-x402`) — ✅ Complete (26 tests)
- **zk-SNARK Anonymous Attestations & Humanity Proofs** (`pallet-gono-privacy`) — ✅ Complete (18 tests)
- **Kleros Decentralized Arbitration Bridge** (`pallet-kleros-bridge`) — ✅ Complete (7 tests)
- **Gono C2PA Bridge Service** (`gono-c2pa-bridge`) — ✅ Complete (1 test)
- **Gono Parachain Runtime** (`gono-runtime`) — ✅ Complete (6 tests, runtime integrity verified)

---

## 🏛️ Gono Sovereign Governance (Three-Tier Architecture)

Per Section 6.1.5 & Section 7 of the Gono Protocol Whitepaper:

```
                       ┌─────────────────────────┐
                       │  Gono Sovereign Model   │
                       └────────────┬────────────┘
                                    │
         ┌──────────────────────────┼──────────────────────────┐
         │                          │                          │
         ▼                          ▼                          ▼
  ┌──────────────┐          ┌──────────────┐          ┌──────────────┐
  │    Tier 1    │          │    Tier 2    │          │    Tier 3    │
  │   Economic   │          │  Integrity   │          │  Autonomous  │
  │  Governance  │          │  Governance  │          │  Arbitration │
  └──────┬───────┘          └──────┬───────┘          └──────┬───────┘
         │                         │                         │
  Conviction Voting        Journalistic Council       Kleros Bridge
  (pallet-democracy)      (pallet-collective)     (pallet-kleros-bridge)
  • 0.5% Deposit           • Top 50 Analysts C_a     • Escalation
  • 7-Day Timelock         • SANUB Parameter Veto    • On-Chain Ruling
```

1. **Tier 1: Core Economic Governance (Token-Weighted)**
   - Powered by `pallet-democracy`, `pallet-preimage`, and `pallet-scheduler`.
   - Conviction Voting with 7-day launch, voting, and enactment timelocks to protect protocol economics.
2. **Tier 2: Technical & Integrity Governance (Reputation-Weighted)**
   - Powered by `pallet-collective<Instance1>` (Journalistic Integrity Council) and `pallet-membership`.
   - Membership automatically granted to the top 50 analysts by Analyst Credit score ($C_a \geq 0.8$). Council holds veto rights over high-impact content and SANUB scoring algorithm updates.
3. **Tier 3: Autonomous Arbitration Governance (Epistemic Engine)**
   - Powered by `pallet-kleros-bridge`.
   - Enables trustless dispute escalation to decentralized Kleros courts for subjective media authenticity rulings.

---

## 📁 Repository Structure

```text
gono-protocol/
├── Cargo.toml                   # Root Cargo Workspace (Unified Polkadot SDK 2606.0.0)
├── AGENTS.md                    # AI Agent Context & Architecture Reference
├── README.md                    # Human-facing project README
├── GonoProtocol_whitepaper.txt  # Full Protocol Whitepaper
│
├── pallets/                     # ═══ Substrate FRAME Pallets ═══
│   ├── store/                  # ERC-7053 Media Receipts, DAG & C2PA Metadata ✅ (14 tests)
│   ├── verify/                 # SANUB Credibility Scoring & Reputation ✅ (13 tests)
│   ├── x402/                   # HTTP 402 State Channel Micropayments ✅ (26 tests)
│   ├── privacy/                # zk-SNARK Verifiers & Anonymous Attestations ✅ (18 tests)
│   └── kleros-bridge/          # Kleros Decentralized Arbitration Bridge ✅ (7 tests)
│
├── chain/                       # ═══ Polkadot SDK Parachain ═══
│   ├── runtime/                # Gono Parachain Runtime (`gono-runtime`) ✅ (6 tests)
│   └── node/                   # Parachain Node Collator CLI
│
├── services/                    # ═══ Off-Chain Microservices ═══
│   └── c2pa-bridge/            # C2PA CLI & Bridge Service (`gono-c2pa`) ✅ (1 test)
│
├── backend/                     # Rust + Actix-web API Service
└── frontend/                    # Next.js 16 + Tailwind CSS 4 Web App
```

---

## 🚀 Quick Start & Testing

### Running the Test Suite (85 Total Passing Tests)

```bash
# Run all 85 unit & integration tests across all workspace crates
$env:SKIP_WASM_BUILD="1"; $env:CARGO_INCREMENTAL="0"; cargo test --workspace --target-dir target-ci

# Check workspace type alignment
$env:SKIP_WASM_BUILD="1"; $env:CARGO_INCREMENTAL="0"; cargo check --workspace --target-dir target-ci
```

### Running Individual Pallet and Service Tests

```bash
# Store Pallet (ERC-7053 & C2PA Signer Registry)
cargo test -p pallet-gono-store

# C2PA Bridge CLI & Service
cargo test -p gono-c2pa-bridge

# Verify Pallet (SANUB)
cargo test -p pallet-gono-verify

# x402 Micropayment Channels
cargo test -p pallet-gono-x402

# Privacy Pallet (zk-SNARK)
cargo test -p pallet-gono-privacy

# Kleros Arbitration Bridge
cargo test -p pallet-kleros-bridge

# Runtime Integrity & Genesis Tests
cargo test -p gono-runtime
```

---

## 🔄 CI/CD Pipeline

The repository utilizes automated GitHub Actions (`.github/workflows/ci.yml`) triggering on pushes to `main` and all pull requests with parallel execution:

| Job | Check | Command |
|---|---|---|
| **Code Formatting** | `check-format` | `cargo fmt --all -- --check` |
| **Clippy Static Analysis** | `lint-clippy` | `cargo clippy --workspace --all-targets -- -D warnings` |
| **Unit & Integration Tests** | `unit-tests` | `cargo test --workspace --locked` |
| **Parachain Runtime Build** | `build-runtime` | `cargo check -p gono-runtime --release` |

Dependency caching is managed via `Swatinem/rust-cache` with isolated cache keys for sub-3-minute workflow execution. Code style is enforced via `.editorconfig` and `rustfmt.toml` adhering to Substrate tab conventions.

---

## 🤖 AI Agent Reference

For AI coding agents: read [`AGENTS.md`](./AGENTS.md) before making any changes. It contains:
- Complete tech stack with exact dependency versions (Polkadot SDK 2606.0.0 / FRAME v48)
- Pallet architecture reference with storage layouts and extrinsic signatures
- Development conventions and testing workflows

---

## 📜 License
Apache-2.0 / Unlicense
