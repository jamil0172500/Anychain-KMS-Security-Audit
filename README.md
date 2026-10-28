# Anychain KMS - Security Audit (Denial of Service)

## 📌 Overview
This repository contains a Proof of Concept (PoC) for a logical vulnerability discovered in the `anychain-kms` crate (Cregis). The vulnerability leads to an Inconsistent Validation, causing a Denial of Service (DoS) for legitimate users during wallet recovery.

## 🚨 Vulnerability Details
- **Target:** `anychain-kms` (BIP39 Mnemonic Validation)
- **Type:** Inconsistent Validation / Denial of Service (DoS)
- **Severity:** Medium / Low
- **Impact:** If a user inputs a valid seed phrase with extra whitespace (e.g., double spaces from a copy-paste error), `from_phrase()` accepts it, but `validate()` rejects it. This inconsistency prevents users from recovering their wallets.

## 🛠️ Proof of Concept (PoC)
The PoC is written in Rust using the standard testing framework. It demonstrates that a valid mnemonic phrase with double spaces fails the `validate()` function, causing a DoS.

### How to run the PoC:
1. Clone the official Anychain repository: `git clone https://github.com/0xcregis/anychain.git`
2. Navigate to the KMS crate: `cd anychain/crates/anychain-kms`
3. Run the test: `cargo test test_inconsistent_validation_causes_dos`

## 💻 Evidence
**Test Code:**
```rust
#[test]
fn test_inconsistent_validation_causes_dos() {
    let phrase_with_double_space = "abandon  abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let result_from_phrase = Mnemonic::from_phrase(phrase_with_double_space, Language::English);
    assert!(result_from_phrase.is_ok());
    let result_validate = Mnemonic::validate(phrase_with_double_space, Language::English);
    assert!(result_validate.is_err());
}
