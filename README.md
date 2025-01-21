# Unified Binary Tree (UBT)

A reference implementation of a unified binary tree using 32-byte keys and values, following the proposed EIP to replace Ethereum's hexary Merkle Patricia Trie (MPT) with a binary structure.

## Overview

This implementation demonstrates a new binary state tree design that aims to:

- Merge account, storage, and code data into a single unified tree structure
- Use 32-byte keys and values throughout
- Provide efficient Merkle proofs
- Support future validity proof systems
- Maintain post-quantum security through hash-based merkleization

## Key Features

- **Binary Structure**: Uses a binary tree with internal nodes having left/right children
- **Stem Nodes**: Groups 256 related values together using a 31-byte stem + 1 byte subindex
- **BLAKE3 Hashing**: Uses BLAKE3 for merkleization (though hash function choice is not final)
- **Co-location**: Related data (account info, initial storage slots, code chunks) are grouped together
- **Simple Design**: Avoids complex extension nodes and RLP encoding

## Understanding the EIP Proposal

The EIP proposes replacing Ethereum's current Merkle Patricia Trie (MPT) with a unified binary tree structure. Key aspects include:

### Motivation
- Enable blocks to be proved with validity proofs for simpler chain verification
- Current MPT design isn't friendly for validity proofs due to:
  - RLP encoding for nodes
  - Keccak as hashing function
  - Being a "tree of trees"
  - Not including account code in state
- Binary structure provides better balance between out-of-circuit and in-circuit proving
- Smaller Merkle proofs compared to hexary structure

### Key Changes
1. **Unified Structure**: 
   - Merges account and storage tries into a single tree
   - Includes chunked contract code in the tree
   - Removes RLP encoding
   - Co-locates related data for efficiency

2. **Data Organization**:
   - Uses 32-byte keys and values
   - First 31 bytes define the entry stem
   - Last byte is the subindex in that stem
   - Groups of 256 keys with same stem are co-located

3. **Node Types**:
   - InternalNode: Contains left and right hash
   - StemNode: Has stem and 256 values
   - LeafNode: Contains 32-byte value or empty
   - EmptyNode: Represents empty subtrees

## Tree Structure

The tree consists of three main node types:

1. **Internal Nodes**: Binary branch nodes with left and right children
2. **Stem Nodes**: Leaf groups containing up to 256 values sharing a common 31-byte prefix
3. **Empty Nodes**: Represent empty branches/leaves

## Usage

```rust
use ubt::{BinaryTree, address_to_32};
use alloy_primitives::{Address, B256};
// Create a new tree
let mut tree = BinaryTree::new();
// Insert a key-value pair
let key = [0xAA; 32];
let value = B256::from([0xBB; 32]);
tree.insert(key, value);
// Get the root hash
let root = tree.root_hash();
```

## Implementation Details

The implementation provides:

- Basic tree operations (insert, root hash computation)
- Node type definitions (Internal, Stem)
- Merkleization rules following the EIP specification
- Helper functions for Ethereum address conversion

## Contributing

This is a reference implementation to demonstrate the concepts in the EIP. Contributions and improvements are welcome.

## License
MIT