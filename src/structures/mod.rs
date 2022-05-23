//! # Structures
//!
//! A library for modeling consensus algorithm structures.

pub mod block;
pub mod blockchain;
pub mod metadata;
pub mod node;
pub mod time;
pub mod vote;

pub use block::Block;
pub use blockchain::Blockchain;
pub use metadata::Metadata;
pub use node::Node;
pub use time::check_clock;
pub use vote::Vote;
