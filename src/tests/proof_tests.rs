use super::*;
use alloy_primitives::B256;

#[test]
fn test_proof_single_stem() {
    let mut tree = BinaryTree::new();
    let key = [0xAA; 32];
    let value = B256::from([0xBB; 32]);
    
    tree.insert(key, value);
    let proof = tree.generate_proof(&key);
    
    // Verify valid proof
    assert!(BinaryTree::verify_proof(&proof, &key, &value, &tree.root_hash()));
    
    // Verify proof fails with wrong value
    let wrong_value = B256::from([0xCC; 32]);
    assert!(!BinaryTree::verify_proof(&proof, &key, &wrong_value, &tree.root_hash()));
}

#[test]
fn test_proof_internal_node() {
    let mut tree = BinaryTree::new();
    
    // Create tree with internal node
    let key1 = [0x00; 32];
    let key2 = [0x80; 32];
    let val1 = B256::from([0x11; 32]);
    let val2 = B256::from([0x22; 32]);
    
    tree.insert(key1, val1);
    tree.insert(key2, val2);
    
    // Generate and verify proof for first key
    let proof1 = tree.generate_proof(&key1);
    assert!(BinaryTree::verify_proof(&proof1, &key1, &val1, &tree.root_hash()));
    
    // Generate and verify proof for second key
    let proof2 = tree.generate_proof(&key2);
    assert!(BinaryTree::verify_proof(&proof2, &key2, &val2, &tree.root_hash()));
}

#[test]
fn test_proof_multiple_levels() {
    let mut tree = BinaryTree::new();
    
    // Create deep tree
    let test_data = vec![
        ([0x00; 32], B256::from([0x11; 32])),
        ([0x40; 32], B256::from([0x22; 32])),
        ([0x80; 32], B256::from([0x33; 32])),
        ([0xC0; 32], B256::from([0x44; 32])),
    ];
    
    for (key, value) in test_data.iter() {
        tree.insert(*key, *value);
    }
    
    // Verify proofs for all values
    for (key, value) in test_data.iter() {
        let proof = tree.generate_proof(key);
        assert!(BinaryTree::verify_proof(&proof, key, value, &tree.root_hash()));
    }
}

#[test]
fn test_proof_same_stem_different_subindex() {
    let mut tree = BinaryTree::new();
    let stem = [0xAA; 31];
    
    // Insert multiple values with same stem
    let mut keys = Vec::new();
    let mut values = Vec::new();
    
    for i in 0..5 {
        let mut key = [0; 32];
        key[..31].copy_from_slice(&stem);
        key[31] = i;
        let value = B256::from([i; 32]);
        
        tree.insert(key, value);
        keys.push(key);
        values.push(value);
    }
    
    // Verify proofs for all values
    for (key, value) in keys.iter().zip(values.iter()) {
        let proof = tree.generate_proof(key);
        assert!(BinaryTree::verify_proof(&proof, key, value, &tree.root_hash()));
    }
}

#[test]
fn test_proof_non_existent_key() {
    let mut tree = BinaryTree::new();
    tree.insert([0xAA; 32], B256::from([0xBB; 32]));
    
    // Generate proof for non-existent key
    let missing_key = [0xCC; 32];
    let proof = tree.generate_proof(&missing_key);
    
    // Should verify as non-existent (with zero value)
    assert!(BinaryTree::verify_proof(
        &proof,
        &missing_key,
        &B256::ZERO,
        &tree.root_hash()
    ));
}

proptest! {
    #[test]
    fn proof_verification_random_trees(
        operations in prop::collection::vec(any::<([u8; 32], [u8; 32])>(), 1..20)
    ) {
        let mut tree = BinaryTree::new();
        
        // Insert random key-value pairs
        for (key, value) in operations.iter() {
            tree.insert(*key, B256::from(*value));
        }
        
        // Verify proof for each inserted pair
        for (key, value) in operations.iter() {
            let proof = tree.generate_proof(key);
            assert!(BinaryTree::verify_proof(
                &proof,
                key,
                &B256::from(*value),
                &tree.root_hash()
            ));
        }
    }
} 