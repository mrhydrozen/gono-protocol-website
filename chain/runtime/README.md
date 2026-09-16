# Runtime

ℹ️ The runtime (in other words, a state transition function), refers to the core logic of the parachain that is
responsible for validating blocks and executing the state changes they define.

💁 The runtime in Gono Protocol is constructed using ready-made FRAME pallets from the
[Polkadot SDK](https://github.com/paritytech/polkadot-sdk), integrated alongside Gono Protocol's 5 domain-specific custom pallets located in [`../../pallets`](../../pallets):
- [`pallet-gono-store`](../../pallets/store): ERC-7053 Media Receipts & C2PA Provenance
- [`pallet-gono-verify`](../../pallets/verify): SANUB Credibility Scoring & Reputation Engine
- [`pallet-gono-x402`](../../pallets/x402): HTTP 402 AI-Native Micropayments
- [`pallet-gono-privacy`](../../pallets/privacy): zk-SNARK Anonymous Attestations & Humanity Proofs
- [`pallet-kleros-bridge`](../../pallets/kleros-bridge): Kleros Decentralized Arbitration Bridge

👉 Learn more about FRAME
[here](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/frame_runtime/index.html).