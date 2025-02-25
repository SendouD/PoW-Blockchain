
use super::*;
use block::Block;
use blockchain::current_timestamp;
use sha2::{Digest, Sha256};

// Block tests
#[test]
fn test_block_creation() {
    let index = 0;
    let previous_hash = "0".to_string();
    let data = "Genesis Block".to_string();
    let difficulty = 2;
    let timestamp = current_timestamp();

    let block = Block::new(
        index,
        previous_hash.clone(),
        data.clone(),
        difficulty,
        timestamp,
    );

    assert_eq!(block.index, index);
    assert_eq!(block.previous_hash, previous_hash);
    assert_eq!(block.data, data);
    assert_eq!(block.timestamp, timestamp);
    assert!(block.hash.starts_with(&"0".repeat(difficulty)));
}

#[test]
fn test_calculate_hash() {
    let block = Block {
        index: 1,
        timestamp: current_timestamp(),
        previous_hash: "abcd1234".to_string(),
        data: "Block Data".to_string(),
        nonce: 42,
        hash: String::new(),
    };

    let expected_hash = {
        let input = format!(
            "{}{}{}{}{}",
            block.index, block.timestamp, block.previous_hash, block.data, block.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        format!("{:x}", hasher.finalize())
    };

    assert_eq!(block.calculate_hash(), expected_hash);
}

#[test]
fn test_mined_hash_is_correct() {
    let mut block = Block {
        index: 4,
        timestamp: current_timestamp(),
        previous_hash: "5678efgh".to_string(),
        data: "Hash Validity Test".to_string(),
        nonce: 0,
        hash: String::new(),
    };

    let mined_hash = block.mine(2);
    assert_eq!(mined_hash, block.calculate_hash());
}

// Blockchain tests
#[test]
fn test_genesis_block_creation() {
    let blockchain = Blockchain::new(4);
    let genesis_block = &blockchain.chain[0];

    assert_eq!(genesis_block.index, 0);
    assert_eq!(genesis_block.previous_hash, "0");
    assert_eq!(genesis_block.data, "Genesis Block".to_string());
    assert!(genesis_block.hash.starts_with("0000")); // Genesis block hash should match difficulty
}

#[test]
fn test_block_addition() {
    let mut blockchain = Blockchain::new(2);
    blockchain.add_block("some data".to_string());
    blockchain.add_block("some more data".to_string());

    assert_eq!(blockchain.chain.len(), 3); // Genesis + 2 blocks
    assert_eq!(blockchain.chain[1].data, "some data");
    assert_eq!(blockchain.chain[2].data, "some more data");
}

#[test]
fn test_proof_of_work_diff_2() {
    let timestamp = current_timestamp();
    let block = Block::new(1, "0".to_string(), "Test Data".to_string(), 2, timestamp);
    assert!(block.hash.starts_with("00")); // Ensure PoW satisfies difficulty
    assert_eq!(block.timestamp, timestamp); // Ensure timestamp is set correctly
}

#[test]
fn test_proof_of_work_diff_4() {
    let block = Block::new(
        1,
        "0".to_string(),
        "Test Data".to_string(),
        4,
        current_timestamp(),
    );
    assert!(block.hash.starts_with("0000")); // Ensure PoW satisfies difficulty
}

#[test]
fn test_chain_validity() {
    let mut blockchain = Blockchain::new(2);
    blockchain.add_block("some data".to_string());
    blockchain.add_block("some more data".to_string());

    assert!(blockchain.is_valid());
}

#[test]
fn test_previous_hash_link() {
    let mut blockchain = Blockchain::new(2);
    blockchain.add_block("some data".to_string());
    blockchain.add_block("some more data".to_string());

    assert_eq!(blockchain.chain[1].previous_hash, blockchain.chain[0].hash);
    assert_eq!(blockchain.chain[2].previous_hash, blockchain.chain[1].hash);
}

#[test]
fn test_balance_initialization_and_tracking() {
    let mut blockchain = Blockchain::new(2);

    // Initially no balances, then add balances
    assert_eq!(blockchain.balances.get("Alice"), None);
    assert_eq!(blockchain.balances.get("Bob"), None);

    blockchain.balances.insert("Alice".to_string(), 200);
    blockchain.balances.insert("Bob".to_string(), 100);

    assert_eq!(*blockchain.balances.get("Alice").unwrap(), 200);
    assert_eq!(*blockchain.balances.get("Bob").unwrap(), 100);
}

#[test]
fn test_transfer_success() {
    let mut blockchain = Blockchain::new(2);
    blockchain.balances.insert("Alice".to_string(), 100);
    blockchain.balances.insert("Bob".to_string(), 50);

    let success = blockchain.transfer_balance("Alice".to_string(), "Bob".to_string(), 30);
    assert!(success); // Transfer should succeed

    assert_eq!(*blockchain.balances.get("Alice").unwrap(), 70); // Alice's balance reduced
    assert_eq!(*blockchain.balances.get("Bob").unwrap(), 80); // Bob's balance increased
}

