use anychain_ethereum::address::EthereumAddress;
use std::str::FromStr;

#[cfg(test)]
mod security_tests {
    use super::*;

    // 🚨 PoC: EIP-55 Checksum Bypass in EthereumAddress
    #[test]
    fn test_checksum_bypass() {
        // ১. সঠিক চেকসামযুক্ত একটি অ্যাড্রেস
        let valid_address = "0x5a2a8410875E882aEe87bF8e5F2e1eDE8810617b";
        
        // ২. একটি ভুল চেকসামযুক্ত ঠিকানা (শেষের 'b' কে বড় হাতের 'B' করা হয়েছে)
        let invalid_checksum_address = "0x5a2a8410875E882aEe87bF8e5F2e1eDE8810617B";

        // ৩. সঠিক ঠিকানা পাস করলে সফল হবে
        let result_valid = EthereumAddress::from_str(valid_address);
        assert!(result_valid.is_ok(), "Valid address should be accepted");

        // ৪. ভুল চেকসামযুক্ত ঠিকানা পাস করলে এটি রিজেক্ট করা উচিত।
        let result_invalid = EthereumAddress::from_str(invalid_checksum_address);
        
        // ❌ বাগ: কিন্তু লাইব্রেরিটি এটিকে সফলভাবে অ্যাক্সেপ্ট করে নিচ্ছে! (কারণ to_lowercase)
        assert!(result_invalid.is_ok(), "Bug: Library accepted an invalid checksum address!");
        
        println!("✅ PoC SUCCESSFUL: Library bypassed EIP-55 checksum validation.");
        println!("Address {} was accepted even though the checksum is invalid.", invalid_checksum_address);
    }
}