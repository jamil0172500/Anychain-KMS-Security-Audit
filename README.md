# Anychain Security Audit Portfolio

## 📌 Overview
This repository contains Proof of Concepts (PoCs) for security vulnerabilities discovered in the `anychain` multi-chain wallet SDK (developed by Cregis). 

## 🚨 Discovered Vulnerabilities

| # | Target | Severity | Vulnerability Type |
|---|--------|----------|--------------------|
| 1 | `anychain-ethereum` | **High** | EIP-55 Checksum Validation Bypass (Loss of Funds) |
| 2 | `anychain-ethereum` | **Medium/High** | DoS via Uncaught Panic in Transaction Parsing |
| 3 | `anychain-kms` | **Medium** | Inconsistent Mnemonic Validation (DoS) |

---

### 🔴 1. High Severity: EIP-55 Checksum Bypass
- **File:** `poc/ethereum/checksum_bypass.rs`
- **Impact:** The SDK converts addresses to lowercase before validation, bypassing EIP-55 checksums. This allows clipboard hijacking malware to redirect funds to an attacker's address without the SDK throwing an error.

### 🟠 2. Medium/High Severity: DoS via String Slicing Panic
- **File:** `poc/ethereum/panic_dos.rs`
- **Impact:** In `legacy.rs`, the `from_str` function uses `&tx[..2]` which panics if the input string is shorter than 2 characters. A malformed string can crash any application or server using this SDK.

### 🟡 3. Medium Severity: KMS Mnemonic Validation DoS
- **File:** `poc/kms_dos.rs`
- **Impact:** `from_phrase()` accepts mnemonics with extra spaces, but `validate()` rejects them. This inconsistency can lock legitimate users out during wallet recovery.

---

## 🛠️ How to Run the PoCs
To verify these vulnerabilities, copy the PoC files into their respective test directories in the official [Anychain repository](https://github.com/0xcregis/anychain) and run:

```bash
cargo test -p anychain-ethereum
cargo test -p anychain-kms
