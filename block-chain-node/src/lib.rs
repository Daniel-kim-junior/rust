pub mod block;
pub mod block_chain;
pub mod transaction;



#[cfg(test)]
mod tests {
    use crate::block::{Block, DIFFICULTY_PREFIX, GENESIS_BLOCK_DATA, GENESIS_BLOCK_INDEX, GENESIS_BLOCK_PREVIOUS_HASH, INCOMPLETE_HASH};
    use crate::block_chain::Blockchain;
    use crate::transaction::Transaction;

    #[test]
    fn test_block_hash_consistency() {
        let mut block1 = Block::new(GENESIS_BLOCK_INDEX, GENESIS_BLOCK_PREVIOUS_HASH.to_string(), Vec::new(), 1633036800);
        
        assert_eq!(block1.hash, INCOMPLETE_HASH);
        block1.mine(DIFFICULTY_PREFIX);
        assert!(block1.is_mined());
        assert!(block1.hash.starts_with(DIFFICULTY_PREFIX));
        assert!(block1.is_valid());
    }

    #[test]
    fn test_block_chain_validity() {
        let mut block_chain = Blockchain::new();
        assert!(block_chain.is_chain_valid());
        let mut block1 = Block::new(GENESIS_BLOCK_INDEX, GENESIS_BLOCK_PREVIOUS_HASH.to_string(), Vec::new(), 1633036800);
        let result = block_chain.add_block(block1.clone());
        assert!(result.is_err());
        block1.mine(DIFFICULTY_PREFIX);
        assert!(block1.is_mined());
        assert!(block1.is_valid());
        assert!(block_chain.add_block(block1).is_ok());
        assert!(block_chain.is_chain_valid());
        let lastBlock = block_chain.get_last_block();
        assert!(lastBlock.is_some());
        let Block {index, previous_hash, transactions, hash, ..} = lastBlock.unwrap();

        assert_eq!(*index, GENESIS_BLOCK_INDEX);
        assert_eq!(previous_hash, GENESIS_BLOCK_PREVIOUS_HASH);
        
        let mut new_block = Block::new(*index + 1, hash.clone(), Vec::new(),1633036801);
        new_block.mine(DIFFICULTY_PREFIX);
        assert!(new_block.is_mined());
        assert!(new_block.is_valid());
        assert!(block_chain.add_block(new_block).is_ok());
        assert!(block_chain.is_chain_valid());

    }
    #[test]
    fn test_transaction() {
        let transaction = Transaction {
            from: "Alice".to_string(),
            to: "Bob".to_string(),
            amount: 100,
        };
        

    }
}
