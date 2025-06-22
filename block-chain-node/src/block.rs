use sha2::{Sha256, Digest};
pub const INCOMPLETE_HASH: &str = "INCOMPLETE";
pub const DIFFICULTY_PREFIX: &str = "0000"; // Example difficulty prefix for mining

#[derive(Debug, Clone)]
pub struct Block {
  pub index: u64,
  pub previous_hash: String,
  pub timestamp: u128,
  pub data: String,
  pub nonce: u64,
  pub hash: String,
}


impl Block {
    pub fn new(index: u64, previous_hash: String, data: String, timestamp: u128) -> Self {
     Self {
        index,
        previous_hash,
        timestamp,
        data,
        nonce: 0,
        hash: INCOMPLETE_HASH.to_string(),
      }
    }
    
    pub fn mine(&mut self, difficulty_prefix: &str) {
      loop {
        let hash = Block::calculate_hash(self.index, &self.previous_hash, self.timestamp, &self.data, self.nonce);
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


      let Block { index, previous_hash, timestamp, data, nonce, hash} = self;
      let calculated_hash = Block::calculate_hash(*index, previous_hash, *timestamp, data, *nonce);
     
      calculated_hash == *hash

    }


    pub fn calculate_hash(index: u64, previous_hash: &str, timestamp: u128, data: &str, nonce: u64) -> String {
      let mut hasher = Sha256::new();
      hasher.update(index.to_string());
      hasher.update(previous_hash);
      hasher.update(timestamp.to_string());
      hasher.update(data);
      hasher.update(nonce.to_string());
      let result = hasher.finalize();
      hex::encode(result)
    }
    
    pub fn is_mined(&self) -> bool {
        self.hash != INCOMPLETE_HASH
    }
}