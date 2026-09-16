# 📡 Gono Protocol Backend Service (gono-backend)

> **REST API Service for Gono Protocol**: Live Substrate RPC integration, ERC-7053 Media Receipts, SANUB Credibility Engine, and x402 Micropayment Gateway.

---

## 1. Overview

The gono-backend crate provides an asynchronous Actix-web service that serves as the bridge between off-chain clients (web browsers, AI agents, automated media publishers) and the on-chain Gono Execution Rail (gono-runtime).

### Key Capabilities

1. **Live Substrate JSON-RPC Client (src/chain.rs)**:
   - Directly queries the Gono node RPC (configured via GONO_RPC_URL, defaults to http://127.0.0.1:9944).
   - Retrieves real-time node health, peer count, syncing status, runtime version, and latest finalized block headers.
   - Built-in graceful fallback to high-fidelity embedded cache when operating in standalone development or testing mode.
2. **ERC-7053 Media Receipt Ledger**:
   - Full content-addressed provenance indexing by CID.
   - DAG revision traversal (parent_cid -> child revisions).
   - Multi-provider decentralized storage metadata tracking (IPFS, Arweave, Filecoin, StorJ).
3. **SANUB Credibility Scoring Engine**:
   - Implements the mathematical formulas from Section 8.2 of the Gono Protocol Whitepaper:
     - Public Belief ( = N_{\text{true}} / (N_{\text{true}} + N_{\text{false}})$)
     - Belief Sigmoid ((B_n) = \frac{1}{1 + e^{-k(B_n - 0.5)}}$)
     - Content Credibility ( = 0.5 \cdot S(B_n) + 0.5 \cdot C_r$)
   - Community verifier voting and analyst review updates with automatic score recalculation.
4. **x402 Micropayment Protocol Gateway (src/x402.rs)**:
   - Implements the machine-to-machine payment protocol from Whitepaper Section 5.4.
   - Issues HTTP-402 challenge quotes with price, recipient address, channel ID, and validity windows.
   - Cryptographically validates off-chain payment voucher tickets against on-chain state channel deposits.

---

## 2. API Endpoints

### Health & Node Status
- GET /api/health — Check backend and Substrate node connectivity, peer count, and block height.

### Media Receipts (ERC-7053)
- GET /api/receipts — List all registered Media Receipts.
- GET /api/receipts/{cid} — Get details of a specific Media Receipt by CID.
- POST /api/receipts — Commit a new Media Receipt on-chain.

### SANUB Verification
- GET /api/verify/{cid} — Retrieve credibility score, public belief, and verifier breakdown for a CID.
- POST /api/verify/{cid}/vote — Cast a community verifier vote ({"positive": true}).

### x402 Micropayments
- GET /api/x402/channels/{id} — Query state channel status (deposit, settled amount, state).
- POST /api/x402/quote — Request HTTP-402 challenge quote for a resource.
- POST /api/x402/verify-ticket — Submit and verify an off-chain payment voucher ticket.

### Protocol Ecosystem
- GET /api/stats — Protocol network stats and dynamic asset counts.
- GET /api/features — Overview of the 5 core Gono modules.
- GET /api/archive — Provenance archive records.
- GET /api/token — GONO tokenomics information.
- GET /api/architecture — 4-layer architecture specification.
- GET /api/use-cases — Primary protocol use cases.

---

## 3. Running & Testing

`ash
# Run unit & integration tests (3 passing)
cargo test --manifest-path backend/Cargo.toml

# Check compilation
cargo check --manifest-path backend/Cargo.toml

# Start the API server
cargo run --manifest-path backend/Cargo.toml
`

Server starts by default at http://127.0.0.1:8080.
