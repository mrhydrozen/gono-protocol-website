# 🌉 Gono C2PA Bridge Service (`gono-c2pa`)

Thin Rust service that connects C2PA media signing & validation → IPFS storage → Gono Chain (`pallet-gono-store::commit_receipt`).

---

## 🛠️ CLI Subcommands

### 1. `sign` — Sign Media with C2PA Provenance
Embeds C2PA claim generator `"Gono Protocol/0.1.0"`, `"c2pa.created"` action assertion, and custom `"org.gono.provenance"` metadata assertion containing the original SHA-256 hash.

```bash
cargo run -p gono-c2pa-bridge --bin gono-c2pa -- sign photo.jpg \
  --cert certs/dev/dev_cert.pem \
  --key certs/dev/dev_key.pem \
  --output photo.signed.jpg
```

### 2. `verify` — Verify C2PA Manifest
Reads and cryptographically validates the manifest embedded inside a media file.

```bash
# Verify with trust check
cargo run -p gono-c2pa-bridge --bin gono-c2pa -- verify photo.signed.jpg

# Verify dev/self-signed certificate
cargo run -p gono-c2pa-bridge --bin gono-c2pa -- verify photo.signed.jpg --allow-untrusted
```

### 3. `upload` — Upload File to IPFS
Uploads a media asset to a local or remote IPFS node (`/api/v0/add`).

```bash
cargo run -p gono-c2pa-bridge --bin gono-c2pa -- upload photo.signed.jpg --ipfs-url http://127.0.0.1:5001
```

### 4. `commit` — Full Pipeline
End-to-end pipeline: signs the file, hashes original content, uploads to IPFS, and prints JSON payload ready for `pallet-gono-store::commit_receipt`.

```bash
cargo run -p gono-c2pa-bridge --bin gono-c2pa -- commit photo.jpg \
  --cert certs/dev/dev_cert.pem \
  --key certs/dev/dev_key.pem
```

---

## 🔑 Generating Development Certificates

Generate a self-signed Ed25519 or ECDSA certificate chain:

```bash
mkdir -p certs/dev
openssl req -x509 -newkey ed25519 \
  -keyout certs/dev/dev_key.pem \
  -out certs/dev/dev_cert.pem \
  -days 365 -nodes \
  -subj "/CN=Gono Dev/O=Gono Protocol/C=BD"
```

---

## 🧪 Testing

```bash
cargo test -p gono-c2pa-bridge
```
