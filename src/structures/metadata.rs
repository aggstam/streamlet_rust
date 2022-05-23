use std::time::Instant;

use super::vote::Vote;

/// This struct represents additional Block information used by the Streamlet consensus protocol.
#[derive(Debug, Clone)]
pub struct Metadata {
    /// Epoch votes
    pub votes: Vec<Vote>,
    /// Block notarization flag
    pub notarized: bool,
    /// Block finalization flag
    pub finalized: bool,
    /// Block creation timestamp
    pub timestamp: Instant,
}

impl Metadata {
    pub fn new() -> Metadata {
        Metadata {
            votes: Vec::new(),
            notarized: false,
            finalized: false,
            timestamp: Instant::now(),
        }
    }
}
