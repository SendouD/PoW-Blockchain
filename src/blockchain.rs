use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::block::Block;

pub fn current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub struct Blockchain {
    pub chain: Vec<Block>,
    pub balances: HashMap<String, i32>, // Stores balances for each account
    pub difficulty: usize,
}

impl Blockchain {
    /**
     * Create a new blockchain with a genesis block of type Blockchain.
     * difficulty is the number of leading zeros required in the hash of a block to be considered valid and it should be set to 4 by default.
     */
    pub fn new(difficulty: usize) -> Self {

        let mut blockchain = Blockchain {
            chain: Vec::new(),
            balances: HashMap::new(),
            difficulty,
        };
        blockchain.create_genesis_block();
        blockchain
    }

    pub fn create_genesis_block(&mut self) {
        let genesis_block = Block::new(
            0,
            "0".to_string(),
            "Genesis Block".to_string(),
            self.difficulty,
            current_timestamp(),
        );
        self.chain.push(genesis_block);
    }

    pub fn last_block(&self) -> &Block {

    self.chain.last().unwrap()
    }

    pub fn add_block(&mut self, data: String) {

        let previous_hash = self.last_block().hash.clone();
        let index = self.last_block().index + 1;
        let timestamp = current_timestamp();
        let new_block = Block::new(index, previous_hash, data, self.difficulty, timestamp);
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        // each block’s hash and its connection to the previous block.
        // Return true if the blockchain is valid, false otherwise.
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            if current_block.previous_hash != previous_block.hash {
                return false;
            }
        }
        if !self.chain.is_empty() {
            let genesis = &self.chain[0];
            if genesis.hash != genesis.calculate_hash() {
                return false;
            }
        }
        
        true

    }

    /**
     * Transfer balance (amount) between two accounts, from sender to receiver.
     */
    pub fn transfer_balance(&mut self, sender: String, receiver: String, amount: i32) -> bool {
        // the amount from the sender to the receiver.
        // If the sender has enough balance, update the balances and return true or
        // if the sender has insufficient balance, return false. Print the appropriate message in each case.
        if amount == 0 || amount < 0 {
            println!("Transfer amount must be greater than zero.");
            return false;
        }
        if sender == receiver {
            println!("Sender and receiver must be different.");
            return true;
        }
        let sender_balance = self.balances.get(&sender).unwrap_or(&0).clone();
        let receiver_balance = self.balances.get(&receiver).unwrap_or(&0).clone();
        if sender_balance < amount {
            println!("Insufficient balance for sender: {}", sender);
            return false;
        }
        self.balances.insert(sender.clone(), sender_balance - amount);
        self.balances.insert(receiver.clone(), receiver_balance + amount);
        true
    }

    pub fn show_balances(&self) {
        println!("Balances: {:?}", self.balances);
    }
}
