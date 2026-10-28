use anychain_kms::bip39::{Mnemonic, Language};

#[cfg(test)]
mod security_tests {
    use super::*;

    // 🚨 PoC: Inconsistent Validation Bug (DoS)
    #[test]
    fn test_inconsistent_validation_causes_dos() {
        // আমরা একটি সঠিক ১২ শব্দের সিড ফ্রেজ নিচ্ছি, তবে প্রথম দুটি শব্দের মাঝে দুটি স্পেস (Double Space) দিয়েছি।
        let phrase_with_double_space = "abandon  abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

        // ১. from_phrase() ফাংশনটি split_whitespace() ব্যবহার করে, তাই এটি বাড়তি স্পেস ইগনোর করে সফলভাবে পাস করবে।
        let result_from_phrase = Mnemonic::from_phrase(phrase_with_double_space, Language::English);
        assert!(result_from_phrase.is_ok(), "from_phrase should accept it and ignore the extra space");

        // ২. কিন্তু validate() ফাংশনটি split(' ') ব্যবহার করে, তাই এটি বাড়তি স্পেসটিকে একটি খালি শব্দ (Empty word) হিসেবে ধরে এরর দেবে।
        let result_validate = Mnemonic::validate(phrase_with_double_space, Language::English);
        
        println!("Validate result: {:?}", result_validate);
        
        // এটি প্রমাণ করে যে একটি ভ্যালিড সিড ফ্রেজও validate() ফাংশনে ফেইল করবে।
        assert!(result_validate.is_err(), "validate should reject it due to the double space, proving the inconsistency");
    }
}