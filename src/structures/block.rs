use std::hash::{Hash, Hasher};

use super::metadata::Metadata;

/// This struct represents a tuple of the form (h, e, txs, metadata).
/// Each blocks parent hash h may be computed simply as a hash of the parent block.
#[derive(Debug, Clone)]
pub struct Block {
    /// Parent hash
    pub h: String,
    /// Epoch number
    pub e: u64,
    /// Transactions payload
    pub txs: Vec<String>,
    /// Additional block information
    pub metadata: Metadata,
}

impl Block {
    pub fn new(h: String, e: u64, txs: Vec<String>) -> Block {
        Block { h, e, txs, metadata: Metadata::new() }
    }

    pub fn signature_encode(&self) -> Vec<u8> {
        let signature = format!("{:?}{:?}{:?}", self.h, self.e, self.txs);
        signature.as_bytes().to_vec()
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.h == other.h && self.e == other.e && self.txs == other.txs
    }
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        (&self.h, &self.e, &self.txs).hash(hasher);
    }
}
