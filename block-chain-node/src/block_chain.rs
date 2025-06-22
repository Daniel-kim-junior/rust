use crate::block::{Block, GENESIS_BLOCK_PREVIOUS_HASH};

pub struct Blockchain {
  pub blocks: Vec<Block>
}


impl Blockchain {

  pub fn new() -> Self {
    let blockchain = Blockchain { blocks: Vec::new() };
    blockchain
  }


  pub fn is_chain_valid(&self) -> bool {
     for i in 1..self.blocks.len() {
      let current_block = &self.blocks[i];
      let prev_block = &self.blocks[i - 1];

      if !current_block.is_valid() {
        return false;
      }
      if current_block.previous_hash != prev_block.hash {
        return false;
      }

      if current_block.index != prev_block.index + 1 {
        return false;
      }
     }

     true
  }

  pub fn add_block(&mut self, block: Block) -> Result<(), String> {
    let last_hash = self.blocks.last().map_or(GENESIS_BLOCK_PREVIOUS_HASH.to_string(), |b| b.hash.clone());
    
    if last_hash != block.previous_hash {
      return Err("Previous hash mismatch".to_string())
    }


    if block.is_valid() {
      self.blocks.push(block);
      Ok(())
    } else {
      Err("Invalid block".to_string())
    }
  }

  pub fn get_last_block(&self) -> Option<&Block> {
    self.blocks.last().map_or(None,|b| Some(b))
  }
}