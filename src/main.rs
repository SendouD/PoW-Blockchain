mod block;
mod blockchain;

use blockchain::Blockchain;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::thread;
use std::time::Duration;

fn main() {
    let mut blockchain = Blockchain::new(4);
    let accounts = vec!["Alice", "Bob", "Charlie", "Dave", "Eve"];
    for account in accounts {
        blockchain.balances.insert(account.to_string(), 10000);
    }


    let mut indexer = 0;
    // Infinite loop to continuously create blocks
    loop {

        // and show the balances on the "blockchain"
         if indexer % 3 == 0 {
             blockchain.transfer_balance("Alice".to_string(), "Bob".to_string(), 10);
                blockchain.transfer_balance("Bob".to_string(), "Charlie".to_string(), 20);
                blockchain.transfer_balance("Charlie".to_string(), "Dave".to_string(), 30);
                blockchain.transfer_balance("Dave".to_string(), "Eve".to_string(), 40);
             blockchain.show_balances();
         }
         print!("Block #{} ", indexer);

        let data: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        // Add a new block to the blockchain with the provided data
        blockchain.add_block(data);

        // Add a small delay to simulate mining time
        thread::sleep(Duration::from_secs(1));
        indexer += 1;
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod extra_tests;
