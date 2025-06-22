pub mod block;

#[cfg(test)]
mod tests {
    use crate::block::{Block, DIFFICULTY_PREFIX, INCOMPLETE_HASH};

    use super::*;

    #[test]
    fn test_block_hash_consistency() {
        let mut block1 = Block::new(0, "0".to_string(), "Genesis Block".to_string(), 
                                1633036800);
        
        assert_eq!(block1.hash, INCOMPLETE_HASH);
        block1.mine(DIFFICULTY_PREFIX);
        assert!(block1.is_mined());
        assert!(block1.hash.starts_with(DIFFICULTY_PREFIX));
        assert!(block1.is_valid());
        
    }
}
