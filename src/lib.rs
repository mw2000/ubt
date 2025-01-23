//! A reference implementation of a unified binary tree using 32-byte keys and values.
//! This follows the EIP proposal to replace the hexary MPT with a binary structure.
//!
//! The tree merges account, storage, and code data into a single map-like data structure.
//! We illustrate how to:
//!  - Represent node types (empty, internal, stem)
//!  - Insert a key-value pair
//!  - Compute the root hash (merkelization)
//!  - Provide helper functions to interoperate with alloy's types
//!
//! # Features
//!
//! - Binary tree structure with 32-byte keys and values
//! - Efficient Merkle proof generation
//! - BLAKE3-based merkleization
//! - Support for account, storage, and code data in a unified structure
//!
//! # Example
//!
//! ```rust
//! use ubt::{BinaryTree, address_to_32};
//! use alloy_primitives::{Address, B256};
//!
//! let mut tree = BinaryTree::new();
//!
//! // Insert account data
//! let address = Address::from([0x42; 20]);
//! let key = address_to_32(address);
//! let value = B256::from([0xFF; 32]);
//!
//! tree.insert(key, value);
//! let root = tree.root_hash();
//! ```

mod node;
mod proof;
mod tree;
mod utils;

pub use node::{InternalNode, Node, StemNode};
pub use tree::BinaryTree;
pub use utils::address_to_32;

#[cfg(test)]
mod tests {
    use super::*;

    mod common;
    mod node_tests;
    mod tree_tests;
    mod utils_tests;
}
