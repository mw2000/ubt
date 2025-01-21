//! A reference implementation of a unified binary tree using 32-byte keys and values.
//! This follows the EIP proposal to replace the hexary MPT with a binary structure.
//!
//! The tree merges account, storage, and code data into a single map-like data structure.
//! We illustrate how to:
//!  - Represent node types (empty, internal, stem)
//!  - Insert a key-value pair
//!  - Compute the root hash (merkelization)
//!  - Provide helper functions to interoperate with alloy's types

mod node;
mod tree;
mod utils;

pub use node::{InternalNode, Node, StemNode};
pub use tree::BinaryTree;
pub use utils::address_to_32;

#[cfg(test)]
mod tests {
    use alloy_primitives::{Address, B256};

    use super::*;

    #[test]
    fn test_insert_and_root_hash() {
        let mut tree = BinaryTree::new();

        // Example: Insert a single key
        let key = [0xAA; 32];
        let value = B256::from([0xBB; 32]);
        tree.insert(key, value);

        let root = tree.root_hash();
        assert_ne!(root, B256::ZERO, "Root should not be empty");
    }

    #[test]
    fn test_two_stems_different_bit() {
        let mut tree = BinaryTree::new();
        let key1 = [0x00; 32];
        let key2 = [0x80; 32]; // differs in the top bit
        let val1 = B256::from([0x11; 32]);
        let val2 = B256::from([0x22; 32]);

        tree.insert(key1, val1);
        tree.insert(key2, val2);

        let root = tree.root_hash();
        assert_ne!(root, B256::ZERO, "Root should not be empty");
    }

    #[test]
    fn test_address_conversion() {
        // Example: Convert an Ethereum address into a 32-byte key
        let addr = Address::from([0x11u8; 20]);
        let arr32 = address_to_32(addr);
        assert_eq!(&arr32[12..], &[0x11u8; 20]);
    }
}
