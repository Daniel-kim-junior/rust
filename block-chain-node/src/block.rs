use sha2::{Sha256, Digest};

use crate::transaction::{Transaction};
pub const INCOMPLETE_HASH: &str = "INCOMPLETE";
pub const DIFFICULTY_PREFIX: &str = "0000"; // Example difficulty prefix for mining
pub const GENESIS_BLOCK_INDEX: u64 = 0;
pub const GENESIS_BLOCK_PREVIOUS_HASH: &str = "0"; // Previous hash for the genesis block
pub const GENESIS_BLOCK_DATA: &str = "Genesis Block";

#[derive(Debug, Clone)]
pub struct Block {
  pub index: u64,
  pub previous_hash: String,
  pub timestamp: u128,
  pub transactions: Vec<Transaction>,
  pub nonce: u64,
  pub hash: String,
}


impl Block {
    pub fn new(index: u64, previous_hash: String, transactions: Vec<Transaction> ,timestamp: u128) -> Self {
     Self {
        index,
        previous_hash,
        timestamp,
        transactions,
        nonce: 0,
        hash: INCOMPLETE_HASH.to_string(),
      }
    }
    
    pub fn mine(&mut self, difficulty_prefix: &str) {
      loop {
        let hash = Block::calculate_hash(self.index, &self.previous_hash, self.timestamp, &self.transactions, self.nonce);
        if hash.starts_with(difficulty_prefix) {
          self.hash = hash;
          break;
        }
        self.nonce += 1;
      }
    }

    pub fn is_valid(&self) -> bool {
      if self.hash == INCOMPLETE_HASH {
        return false;
      }

      if  !self.hash.starts_with(DIFFICULTY_PREFIX) {
        return false;
      }


      let Block { index, previous_hash, timestamp, transactions, nonce, hash} = self;
      let calculated_hash = Block::calculate_hash(*index, previous_hash, *timestamp, transactions, *nonce);
     
      calculated_hash == *hash

    }


    pub fn calculate_hash(index: u64, previous_hash: &str, timestamp: u128, transactions: &[Transaction], nonce: u64) -> String {
      let tx_data: String = transactions.iter()
      .map(|tx| format!("{}->{}:{}", tx.from, tx.to, tx.amount))
      .collect();
      let content: String = format!("{}{}{}{}{}", index, previous_hash, timestamp, tx_data, nonce);


      let mut hasher = Sha256::new();
      hasher.update(content.as_bytes());
      let result = hasher.finalize();
      hex::encode(result)
    }
    
    pub fn is_mined(&self) -> bool {
        self.hash != INCOMPLETE_HASH
    }
}