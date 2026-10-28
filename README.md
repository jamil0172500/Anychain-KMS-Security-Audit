# Anychain Security Audit - Vulnerability Reports

## 📌 Overview
This repository contains detailed Proof of Concepts (PoCs) for vulnerabilities discovered in the `anychain` multi-chain wallet SDK (developed by Cregis). 

During the security assessment, two distinct vulnerabilities were identified: one **High Severity** issue that could lead to a loss of funds, and one **Medium/Low Severity** issue causing a Denial of Service (DoS).

---

## 🚨 Finding 1: High Severity - EIP-55 Checksum Validation Bypass
- **Target:** `anychain-ethereum` crate
- **File:** `crates/anychain-ethereum/src/address.rs`
- **Impact:** Loss of user funds. The library accepts mixed-case Ethereum addresses with invalid EIP-55 checksums. If a user's clipboard is hijacked by malware (changing the case of a letter), the SDK will not reject the transaction, sending funds to an incorrect address.
- **PoC Code:** [`poc/ethereum/ethereum_poc.rs`](poc/ethereum/ethereum_poc.rs)

### How to Run (Finding 1):
```bash
cd crates/anychain-ethereum
cargo test test_checksum_bypass
