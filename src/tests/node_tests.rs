use super::*;
use alloy_primitives::B256;

#[test]
fn test_empty_tree_hash() {
    let tree = BinaryTree::new();
    assert_eq!(tree.root_hash(), B256::ZERO);
}

#[test]
fn test_stem_node_hash_consistency() {
    let mut tree = BinaryTree::new();
    let key = [0xAA; 32];
    let value = B256::from([0xBB; 32]);

    // Insert and get first hash
    tree.insert(key, value);
    let hash1 = tree.root_hash();

    // Create new tree with same data
    let mut tree2 = BinaryTree::new();
    tree2.insert(key, value);
    let hash2 = tree2.root_hash();

    // Hashes should be deterministic
    assert_eq!(hash1, hash2);
}

#[test]
fn test_internal_node_hash() {
    let mut tree = BinaryTree::new();

    // Create an internal node by inserting two values that differ in first bit
    let key1 = [0x00; 32];
    let key2 = [0x80; 32];
    let val1 = B256::from([0x11; 32]);
    let val2 = B256::from([0x22; 32]);

    tree.insert(key1, val1);
    let hash1 = tree.root_hash();

    tree.insert(key2, val2);
    let hash2 = tree.root_hash();

    // Hash should change after second insert
    assert_ne!(hash1, hash2);
    assert_ne!(hash2, B256::ZERO);
}

#[test]
fn test_hash_ordering() {
    let mut tree1 = BinaryTree::new();
    let mut tree2 = BinaryTree::new();

    // Insert same key-value pairs in different order
    let pairs = [
        ([0x11; 32], B256::from([0xAA; 32])),
        ([0x22; 32], B256::from([0xBB; 32])),
        ([0x33; 32], B256::from([0xCC; 32])),
    ];

    for (key, value) in pairs.iter() {
        tree1.insert(*key, *value);
    }

    for (key, value) in pairs.iter().rev() {
        tree2.insert(*key, *value);
    }

    // Final state should have same hash regardless of insertion order
    assert_eq!(tree1.root_hash(), tree2.root_hash());
}
