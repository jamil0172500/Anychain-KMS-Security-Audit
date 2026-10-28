use anychain_ethereum::transaction::EthereumTransaction;
use anychain_ethereum::network::Sepolia;
use std::str::FromStr;

#[test]
fn test_from_str_panic() {
    let malicious_input = "0"; 
    
    let result = std::panic::catch_unwind(|| {
        EthereumTransaction::<Sepolia>::from_str(malicious_input)
    });

    assert!(result.is_err(), "Bug: Library panicked on short input!");
}
