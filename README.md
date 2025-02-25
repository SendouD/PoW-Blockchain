

## Description: a State Machine-Based Blockchain with Balance transfers

## Overview
 a simplified version of a Proof of Work (PoW) blockchain. The main goal of this assignment is to show the understanding you have of the core principles of blockchain technology, including block creation, proof of work, and validation of blocks. Also show some basic balance tracking and implementing transactions between accounts.

1. Blocks are added to the chain through a Proof-of-Work process, and each block must be validated based on its contents.
2. The blockchain should support transfers between accounts with basic balance tracking (simple logging).
3. Ensure that the blockchain’s integrity is maintained through proper hashing.

## Project Structure

### 1. Block Structure

A block should contain:

- Index: The position of the block in the chain.
- Timestamp: When the block was created.
- Previous Hash: The hash of the previous block in the chain.
- Data: The contents or payload of the block.
- Nonce: A number used to solve the Proof-of-Work (PoW) problem during block mining.
- Hash: The unique cryptographic hash of the block, calculated based on its contents (index, timestamp, previous_hash, data, nonce).

### 2. Blockchain Structure

- Start with a genesis block.
- Store multiple blocks in sequence.
- Manage balances for accounts.
- Add blocks once they are mined after proof of work is executed.

### 3. Proof of Work

The Proof of Work algorithm is a simple computational puzzle that ensures blocks are added with a sufficient level of difficulty. It should:

- Find a proof such that, when hashed with the previous proof, it produces a hash starting with a number of leading zeros specified by the difficulty.

### 4. Balance Tracking and Transfers

- Maintain balances for different accounts in a Map.
- Allow transferring balances between accounts only if the sender has sufficient funds.

## Tests

A set of initial tests is provided to help you validate your solution.

To run the tests:

```bash
$ cargo test
```
Good luck, and have fun coding! 🚀
