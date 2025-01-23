use super::*;
use alloy_primitives::B256;
use proptest::prelude::*;

use crate::tests::common::test_utils::*;

// Property-based tests
proptest! {
    #[test]
    fn doesnt_crash_on_random_inserts(
        keys in prop::collection::vec(any::<[u8; 32]>(), 0..100),
        values in prop::collection::vec(any::<[u8; 32]>(), 0..100)
    ) {
        let mut tree = BinaryTree::new();
        for (key, value) in keys.into_iter().zip(values) {
            tree.insert(key, B256::from(value));
        }
    }

    #[test]
    fn maintains_consistency_after_multiple_inserts(
        operations in prop::collection::vec(any::<([u8; 32], [u8; 32])>(), 1..50)
    ) {
        let mut tree = BinaryTree::new();

        for (key, value) in operations {
            tree.insert(key, B256::from(value));
            // Verify tree invariants after each insert
            assert!(verify_tree_invariants(&tree));
        }
    }
}

// Basic functionality tests
#[test]
fn test_empty_tree() {
    let tree = BinaryTree::new();
    assert_eq!(tree.root_hash(), B256::ZERO);
}

#[test]
fn test_single_insert() {
    let mut tree = BinaryTree::new();
    let key = [0xAA; 32];
    let value = B256::from([0xBB; 32]);

    tree.insert(key, value);

    let root_hash = tree.root_hash();
    assert_ne!(root_hash, B256::ZERO);
}

#[test]
fn test_multiple_inserts_same_stem() {
    let mut tree = BinaryTree::new();
    let stem = [0xAA; 31];

    // Insert multiple values with same stem but different subindices
    for i in 0..5 {
        let mut key = [0; 32];
        key[..31].copy_from_slice(&stem);
        key[31] = i;
        tree.insert(key, B256::from([i; 32]));
    }

    // Verify tree structure
    if let Node::Stem(stem_node) = &tree.root {
        assert_eq!(stem_node.stem, stem);
        for i in 0..5 {
            assert_eq!(stem_node.values[i as usize], Some(B256::from([i; 32])));
        }
        for i in 5..256 {
            assert_eq!(stem_node.values[i], None);
        }
    } else {
        panic!("Expected stem node");
    }
}

#[test]
fn test_collision_handling() {
    let mut tree = BinaryTree::new();

    // Insert two keys that differ in their first bit
    let key1 = [0x00; 32];
    let key2 = [0x80; 32];

    tree.insert(key1, B256::from([0x11; 32]));
    tree.insert(key2, B256::from([0x22; 32]));

    // Verify we have an internal node at the root
    if let Node::Internal(internal) = &tree.root {
        assert!(matches!(internal.left, Node::Stem(_)));
        assert!(matches!(internal.right, Node::Stem(_)));
    } else {
        panic!("Expected internal node");
    }
}

#[test]
fn test_merkle_proof_verification() {
    // Add comprehensive merkle proof tests
}
