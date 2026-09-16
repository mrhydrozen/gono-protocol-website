# `pallet-gono-verify`

Substrate FRAME pallet implementing **Section 8.2 of the Gono Protocol Whitepaper (SANUB Framework: Sharing and Analyzing News Using Blockchain)**.

## Features
- **Deterministic Fixed-Point Math (`sp_runtime::FixedU128`)**:
  - Public Belief $B_n$ (Eq 2)
  - Content Importance $I_n$ (Eq 3)
  - Belief Sigmoid $S(B_n)$ (Eq 4) using `no_std` Taylor expansion
  - Analyst Positive Credit $T_p$ (Eq 5)
  - Analyst Credit with Asymmetric Punishment $C_a$ (Eq 6)
  - Reporter Credit $C_r$ (Eq 7)
  - Content Credibility Score $C_n$ (Eq 8)
- **Role-Based Extrinsics**:
  - `register_content(origin, cid)`
  - `vote_as_verifier(origin, cid, score_binary)`
  - `submit_analyst_review(origin, cid, verdict, stake)`
  - `finalize_content_score(origin, cid)`
- **Full Documentation**: See [`AGENTS.md`](../../AGENTS.md) at workspace root for detailed specifications and AI agent interaction loops.

## Testing
```bash
cargo test -p pallet-gono-verify
```
