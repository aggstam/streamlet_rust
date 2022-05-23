use super::block::Block;

/// This struct represents a tuple of the form (vote, B, id).
#[derive(Debug, Clone, PartialEq)]
pub struct Vote {
    /// signed block
    pub vote: Vec<u8>,
    /// block to vote on
    pub block: Block,
    /// node id
    pub id: u64,
}

impl Vote {
    pub fn new(vote: Vec<u8>, block: Block, id: u64) -> Vote {
        Vote { vote, block, id }
    }
}
