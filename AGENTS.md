# 🤖 AGENTS.md — Gono Protocol AI Agent Context File

> **Purpose**: This file is the canonical reference for AI coding agents (Antigravity, Cursor, Copilot, etc.) working on the Gono Protocol codebase. Read this file FIRST before making any changes. It contains the project structure, tech stack, implementation status, architectural decisions, dependency constraints, and conventions that every agent session needs to understand.

> **Last Updated**: 2026-08-15

---

## 1. Project Identity

| Field | Value |
|-------|-------|
| **Name** | Gono Protocol |
| **Description** | Provenance infrastructure for humans and AI — a modular blockchain built on Substrate as a Polkadot Parachain |
| **Whitepaper** | [`GonoProtocol_whitepaper.txt`](./GonoProtocol_whitepaper.txt) (full protocol spec) |
| **Analysis** | [`gono_protocol_analysis.md`](./gono_protocol_analysis.md) (study prerequisites & architecture deep-dive) |
| **Repository** | `https://github.com/Meherajs/gono-protocol-website` |
| **Branch** | `main` |
| **License** | Apache-2.0 |

---

## 2. Tech Stack & Versions

### Blockchain / Backend

| Dependency | Version | Notes |
|------------|---------|-------|
| **Rust Edition** | 2021 | All crates use `edition = "2021"` |
| **polkadot-sdk** | `2606.0.0` | Umbrella crate for runtime and parachain tooling |
| **parity-scale-codec** | `3.7.5` | Workspace alias: `codec`, features: `derive`, `max-encoded-len` |
| **scale-info** | `2.11.6` | Feature: `derive` |
| **frame-support** | `48.0.0` | Aligned with polkadot-sdk 2606.0.0 |
| **frame-system** | `48.0.0` | Aligned with polkadot-sdk 2606.0.0 |
| **pallet-balances** | `50.0.0` | Aligned with polkadot-sdk 2606.0.0 |
| **sp-core** | `43.0.0` | Aligned with polkadot-sdk 2606.0.0 |
| **sp-io** | `48.0.0` | Aligned with polkadot-sdk 2606.0.0 |
| **sp-runtime** | `48.0.0` | Aligned with polkadot-sdk 2606.0.0 |
| **Actix-web** | `4.0` | Backend REST API (`backend/` crate, separate from workspace) |

> **⚠️ CRITICAL**: `sp-core`, `sp-io`, and `sp-runtime` versions MUST stay aligned with `frame-support` and `polkadot-sdk`. When upgrading, run `cargo tree -i sp-storage` and `cargo tree -i sp-io` to verify no duplicate versions exist.

### Frontend

| Dependency | Version |
|------------|---------|
| **Next.js** | 16 |
| **React** | 19 |
| **Tailwind CSS** | 4.0 |
| **Deployment** | Vercel (`vercel.json` in root) |

---

## 3. Repository Structure

```
gono-protocol/
├── Cargo.toml                     # Root workspace configuration
├── AGENTS.md                      # THIS FILE — AI agent reference
├── README.md                      # Human-facing project README
├── GonoProtocol_whitepaper.txt    # Full protocol whitepaper
├── gono_protocol_analysis.md      # Whitepaper deep analysis & learning roadmap
├── vercel.json                    # Vercel deployment config (frontend)
│
├── pallets/                       # ═══ Substrate FRAME Pallets ═══
│   ├── store/                     # [142] ERC-7053 Media Receipts & CID Provenance
│   │   ├── Cargo.toml             # Deps: frame-support, frame-system, codec, scale-info
│   │   └── src/ (lib.rs, types.rs, mock.rs, tests.rs) -> 11 passing tests
│   │
│   ├── verify/                    # [144] SANUB Credibility Scoring & Reputation Math
│   │   ├── Cargo.toml             # Deps: + sp-runtime (for FixedU128)
│   │   └── src/ (lib.rs, types.rs, math.rs, mock.rs, tests.rs) -> 13 passing tests
│   │
│   ├── x402/                      # [148] HTTP 402 State Channel Micropayments
│   │   ├── Cargo.toml             # Deps: frame-support, frame-system, sp-runtime, sp-core, pallet-balances
│   │   └── src/ (lib.rs, types.rs, mock.rs, tests.rs) -> 26 passing tests
│   │
│   ├── privacy/                   # [146] ZK-SNARK Attestation & Proof Verifier
│   │   ├── Cargo.toml             # Deps: frame-support, frame-system, codec, scale-info, sp-core, sp-runtime
│   │   └── src/ (lib.rs, types.rs, verifier.rs, mock.rs, tests.rs) -> 18 passing tests
│   │
│   └── kleros-bridge/             # [156] Kleros Decentralized Arbitration Bridge (Tier 3 Governance Stub)
│       ├── Cargo.toml             # Deps: frame-support, frame-system, codec, scale-info, sp-runtime
│       └── src/ (lib.rs, mock.rs, tests.rs) -> 7 passing tests
│
├── chain/                         # ═══ Polkadot SDK Parachain ═══
│   ├── runtime/                   # Gono Parachain Runtime (`gono-runtime`)
│   │   ├── Cargo.toml             # Composes all Gono pallets + Three-Tier Governance
│   │   └── src/                   # apis.rs, benchmarks.rs, configs/, weights/, lib.rs -> 6 passing tests
│   └── node/                      # Collator Node CLI
│       ├── Cargo.toml
│       └── src/                   # chain_spec.rs, cli.rs, command.rs, rpc.rs, service.rs
│
├── backend/                       # ═══ REST API Service ═══
│   ├── Cargo.toml                 # Standalone crate (NOT in workspace)
│   └── src/main.rs                # Actix-web API server
│
└── frontend/                      # ═══ Next.js Web App ═══
    ├── app/                       # App Router (/ and /whitepaper routes)
    ├── components/                # React UI components
    └── public/                    # Static assets
```

---

## 4. Workspace Configuration

### Active Members (compile & test)
```toml
members = [
    "chain/runtime",
    "pallets/store",
    "pallets/verify",
    "pallets/privacy",
    "pallets/x402",
    "pallets/kleros-bridge",
]
```

### Excluded Members
- `"chain/node"` — to be updated with collator configuration for `gono-runtime`.

---

## 5. Pallet & Runtime Implementation Status

| Pallet / Crate | Crate Name | Status | Files | Tests |
|----------------|-----------|--------|-------|-------|
| **store** | `pallet-gono-store` | ✅ **COMPLETE** | `lib.rs`, `types.rs`, `mock.rs`, `tests.rs` | 14 passing |
| **verify** | `pallet-gono-verify` | ✅ **COMPLETE** | `lib.rs`, `types.rs`, `math.rs`, `mock.rs`, `tests.rs` | 13 passing |
| **x402** | `pallet-gono-x402` | ✅ **COMPLETE** | `lib.rs`, `types.rs`, `mock.rs`, `tests.rs` | 26 passing |
| **privacy** | `pallet-gono-privacy` | ✅ **COMPLETE** | `lib.rs`, `types.rs`, `verifier.rs`, `mock.rs`, `tests.rs` | 18 passing |
| **kleros-bridge** | `pallet-kleros-bridge` | ✅ **COMPLETE** | `lib.rs`, `mock.rs`, `tests.rs` | 7 passing |
| **c2pa-bridge** | `gono-c2pa-bridge` | ✅ **COMPLETE** | `main.rs`, `sign.rs`, `verify.rs`, `ipfs.rs`, `commit.rs`, `tests/` | 1 passing |
| **runtime** | `gono-runtime` | ✅ **COMPLETE** | `lib.rs`, `configs/mod.rs`, `apis.rs`, `weights/` | 6 passing (incl. runtime integrity) |
| **Total Test Suite** | — | ✅ **85 PASSING** | — | **85 passing** |

---

## 6. Architecture & Governance Reference

### 6.1 Gono Sovereign Governance (Whitepaper Section 6.1.5)

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

1. **Tier 1 (Economic Governance)**:
   - Implemented via `pallet_democracy`, `pallet_preimage`, and `pallet_scheduler`.
   - Token-weighted Conviction Voting with 7-day launch, voting, and enactment timelocks.
2. **Tier 2 (Technical and Integrity Governance)**:
   - Implemented via `pallet_collective<Instance1>` (Journalistic Integrity Council) and `pallet_membership<Instance1>`.
   - Meritocratic membership (top 50 analysts by $C_a \geq 0.8$) with proposal veto rights.
3. **Tier 3 (Decentralized Arbitration)**:
   - Implemented via `pallet_kleros_bridge`.
   - Provides on-chain dispute escalation (`escalate_dispute`) and root/oracle ruling execution (`submit_ruling`).

### 6.2 Cross-Pallet Integration

- **`verify → store`**: Decoupled via the `ContentInspector` trait. In `chain/runtime/src/configs/mod.rs`, the `GonoStoreInspector` adapter inspects `pallet_gono_store::Receipts` to verify that content CIDs exist on-chain prior to reputation scoring.
- **`privacy → governance`**: Zero-knowledge proof attestations (`CredentialVerification`) gate journalist and analyst credentials.

---

## 7. Testing, Linting & CI/CD Instructions

The repository runs an automated GitHub Actions CI pipeline (`.github/workflows/ci.yml`) on every push to `main` and all pull requests:

```bash
# 1. Verify code formatting (Substrate tab convention)
cargo fmt --all -- --check

# 2. Run Clippy static analysis with zero warnings tolerated
SKIP_WASM_BUILD=1 cargo clippy --workspace --all-targets --locked -- -D warnings

# 3. Run all 81 unit & integration tests with locked dependencies
SKIP_WASM_BUILD=1 CARGO_INCREMENTAL=0 cargo test --workspace --locked --target-dir target-ci

# 4. Verify Parachain Runtime release build
SKIP_WASM_BUILD=1 CARGO_INCREMENTAL=0 cargo check -p gono-runtime --release --locked --target-dir target-ci
```

---

## 8. Next Implementation Priorities

1. **`chain/node`** — Wire collator node CLI with `gono-runtime`
2. **Backend API Integration** — Connect Actix-web backend with subxt client queries to `gono-runtime`
3. **Frontend Dashboard** — Connect Web3 interface to live chain state
