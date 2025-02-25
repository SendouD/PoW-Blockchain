use super::*;
use block::Block;
use blockchain::current_timestamp;
use sha2::{Digest, Sha256};

#[test]
fn test_insufficent_balance() {
    let mut blockchain = Blockchain::new(4);
    let accounts = vec!["Alice", "Bob", "Charlie", "Dave", "Eve"];
    for account in accounts {
        blockchain.balances.insert(account.to_string(), 10000);
    }

    blockchain.transfer_balance("Alice".to_string(), "Bob".to_string(), 10001);
    assert_eq!(blockchain.balances.get("Alice").unwrap(), &10000);
    assert_eq!(blockchain.balances.get("Bob").unwrap(), &10000);
}

#[test]
fn test_invalid_chain() {
    let mut blockchain = Blockchain::new(2);
    blockchain.add_block("1".to_string());
    blockchain.add_block("2".to_string());    
    blockchain.chain[1].data = "Tampered Block".to_string();
    assert!(!blockchain.is_valid());
}
#[test]
fn test_invalid_genesis_block() {
    
    let mut blockchain = Blockchain::new(2);
    print!("Before tampering: {}",blockchain.chain[0].data);
    
    blockchain.chain[0].data = "Tampered Genesis Block".to_string();
    assert!(!blockchain.is_valid());
}
#[test]
fn test_zero_negative_transfer() {
    let mut blockchain = Blockchain::new(2);
    blockchain.balances.insert("Alice".to_string(), 100);
    blockchain.balances.insert("Bob".to_string(), 100);

    let success = blockchain.transfer_balance("Alice".to_string(), "Bob".to_string(), 0);
    assert!(!success);
    let neg_success=blockchain.transfer_balance("Alice".to_string(), "Bob".to_string(), -1);
    assert!(!neg_success);
    assert_eq!(*blockchain.balances.get("Alice").unwrap(), 100);
    assert_eq!(*blockchain.balances.get("Bob").unwrap(), 100);
}


#[test]
fn test_self_transfer() {
    let mut blockchain = Blockchain::new(2);
    blockchain.balances.insert("Alice".to_string(), 100);

    let success = blockchain.transfer_balance("Alice".to_string(), "Alice".to_string(), 50);
    assert!(success);
    assert_eq!(*blockchain.balances.get("Alice").unwrap(), 100);
}
#[test]
fn test_insufficient_balance() {
    let mut blockchain = Blockchain::new(2);
    blockchain.balances.insert("Alice".to_string(), 50);
    blockchain.balances.insert("Bob".to_string(), 100);
    
    let success = blockchain.transfer_balance("Alice".to_string(), "Bob".to_string(), 100);
    assert!(!success); 

    assert_eq!(blockchain.balances.get("Alice").unwrap(), &50);
    assert_eq!(blockchain.balances.get("Bob").unwrap(), &100);
}

#[test]
fn test_adding_multiple_blocks() {
    let mut blockchain = Blockchain::new(2);
    blockchain.add_block("Block 1 data".to_string());
    blockchain.add_block("Block 2 data".to_string());
    blockchain.add_block("Block 3 data".to_string());

    assert_eq!(blockchain.chain.len(), 4); 
    assert_eq!(blockchain.chain[1].data, "Block 1 data");
    assert_eq!(blockchain.chain[2].data, "Block 2 data");
    assert_eq!(blockchain.chain[3].data, "Block 3 data");
}

#[test]
fn test_blockchain_validity_after_adding_multiple_blocks() {
    let mut blockchain = Blockchain::new(2);
    blockchain.add_block("Block 1 data".to_string());
    blockchain.add_block("Block 2 data".to_string());
    
    assert!(blockchain.is_valid()); 
    
    blockchain.chain[1].data = "Tampered Block".to_string(); 
    
    assert!(!blockchain.is_valid()); 
}

#[test]
fn test_balances_update_correctly_after_transfers() {
    let mut blockchain = Blockchain::new(2);
    blockchain.balances.insert("Alice".to_string(), 500);
    blockchain.balances.insert("Bob".to_string(), 300);

    blockchain.transfer_balance("Alice".to_string(), "Bob".to_string(), 100); 
    blockchain.transfer_balance("Bob".to_string(), "Alice".to_string(), 50);  

    assert_eq!(*blockchain.balances.get("Alice").unwrap(), 450); 
    assert_eq!(*blockchain.balances.get("Bob").unwrap(), 350);   
}

#[test]
fn test_chain_tampering_with_multiple_blocks() {
    let mut blockchain = Blockchain::new(2);
    blockchain.add_block("Block 1 data".to_string());
    blockchain.add_block("Block 2 data".to_string());
    blockchain.add_block("Block 3 data".to_string());
    blockchain.chain[1].data = "Tampered Block".to_string();

    assert!(!blockchain.is_valid()); 
}