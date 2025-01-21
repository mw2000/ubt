use alloy_primitives::B256;

use crate::node::{InternalNode, Node, StemNode};

/// Our top-level binary tree wrapper. Holds the root `Node`.
#[derive(Clone, Debug)]
pub struct BinaryTree {
    pub root: Node,
}

impl Default for BinaryTree {
    fn default() -> Self {
        Self { root: Node::Empty }
    }
}

impl BinaryTree {
    /// Creates a new empty tree.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert a 32-byte key with a 32-byte value (B256) into the tree,
    /// returning a reference to self for chaining.
    ///
    /// - `key`: 32-byte key, where the first 31 bytes are the "stem"
    ///          and the last byte is the "subindex".
    /// - `value`: the 32-byte value to store.
    pub fn insert(&mut self, key: [u8; 32], value: B256) -> &mut Self {
        self.root = Self::insert_recursive(
            std::mem::replace(&mut self.root, Node::Empty),
            &key,
            value,
            0,
        );
        self
    }

    /// Compute the merkle root of the entire tree.
    /// Follows the hashing rules from the proposal.
    pub fn root_hash(&self) -> B256 {
        Self::hash_node(&self.root)
    }

    // ------------------------
    // Internal logic
    // ------------------------

    /// Recursive helper for insertion.
    ///
    /// `depth` tracks which bit of the stem we are branching on
    /// when we are in an `InternalNode`.
    fn insert_recursive(node: Node, key: &[u8; 32], value: B256, depth: usize) -> Node {
        if depth >= 248 {
            return node;
        }

        match node {
            Node::Empty => Self::create_stem_node(key, value),
            Node::Stem(existing_stem_box) => {
                let existing_stem_node = *existing_stem_box;
                if existing_stem_node.stem == key[..31] {
                    let mut updated = existing_stem_node;
                    updated.values[key[31] as usize] = Some(value);
                    Node::Stem(Box::new(updated))
                } else {
                    let bit_index = Self::bit_of(&key[..31], depth);
                    let existing_bit = Self::bit_of(&existing_stem_node.stem, depth);

                    if bit_index == existing_bit {
                        let child = Self::insert_recursive(
                            Node::Stem(Box::new(existing_stem_node)),
                            key,
                            value,
                            depth + 1,
                        );
                        if bit_index == 0 {
                            Node::Internal(Box::new(InternalNode {
                                left: child,
                                right: Node::Empty,
                            }))
                        } else {
                            Node::Internal(Box::new(InternalNode {
                                left: Node::Empty,
                                right: child,
                            }))
                        }
                    } else {
                        let new_stem = Self::create_stem_node(key, value);

                        if bit_index == 0 && existing_bit == 1 {
                            Node::Internal(Box::new(InternalNode {
                                left: new_stem,
                                right: Node::Stem(Box::new(existing_stem_node)),
                            }))
                        } else {
                            Node::Internal(Box::new(InternalNode {
                                left: Node::Stem(Box::new(existing_stem_node)),
                                right: new_stem,
                            }))
                        }
                    }
                }
            }
            Node::Internal(mut internal_box) => {
                if Self::bit_of(&key[..31], depth) == 0 {
                    internal_box.left =
                        Self::insert_recursive(internal_box.left, key, value, depth + 1);
                } else {
                    internal_box.right =
                        Self::insert_recursive(internal_box.right, key, value, depth + 1);
                }
                Node::Internal(internal_box)
            }
        }
    }

    /// Create a new StemNode for a given key and value. Subindex is key[31].
    fn create_stem_node(key: &[u8; 32], value: B256) -> Node {
        let mut stem_node = StemNode {
            stem: [0u8; 31],
            values: [None; 256],
        };
        stem_node.stem.copy_from_slice(&key[..31]);
        stem_node.values[key[31] as usize] = Some(value);
        Node::Stem(Box::new(stem_node))
    }

    /// Hash a node according to the EIP rules:
    ///  - Empty node => 32 bytes of zero
    ///  - Internal node => `blake3(left_hash || right_hash)`
    ///  - Stem node => `blake3(stem || 0x00 || subroot)`
    ///  - Leaf => `blake3(value)` (but in code we store leaves in `StemNode.values`)
    fn hash_node(node: &Node) -> B256 {
        match node {
            Node::Empty => B256::ZERO,
            Node::Internal(internal) => {
                let left_hash = Self::hash_node(&internal.left);
                let right_hash = Self::hash_node(&internal.right);
                Self::blake3_hash_64(&left_hash.0, &right_hash.0)
            }
            Node::Stem(stem_node) => {
                let subroot = Self::merkleize_leaves(&stem_node.values);
                let mut buf = [0u8; 64];
                buf[..31].copy_from_slice(&stem_node.stem);
                buf[31] = 0x00;
                buf[32..64].copy_from_slice(&subroot.0);
                Self::blake3_hash_64(&buf[..32], &buf[32..64])
            }
        }
    }

    /// Pairwise-merkleize 256 leaves, each is either None or Some(B256).
    /// We treat `None` as a 32-byte zero value (like empty leaf).
    fn merkleize_leaves(values: &[Option<B256>; 256]) -> B256 {
        let mut level = Vec::with_capacity(256);
        let mut next_level = Vec::with_capacity(128);

        for v in values.iter() {
            level.push(match v {
                Some(val) => B256::from(blake3::hash(&val.0).as_bytes()),
                None => B256::ZERO,
            });
        }

        let mut width = 256;
        while width > 1 {
            next_level.clear();
            for i in (0..width).step_by(2) {
                next_level.push(Self::blake3_hash_64(&level[i].0, &level[i + 1].0));
            }
            std::mem::swap(&mut level, &mut next_level);
            width /= 2;
        }

        level[0]
    }

    /// Helper: compute the bit at position `depth` in a 31-byte slice (248 bits).
    #[inline]
    fn bit_of(stem: &[u8], depth: usize) -> u8 {
        let byte = stem[depth >> 3];
        (byte >> (7 - (depth & 7))) & 1
    }

    /// Hash 64 bytes with BLAKE3 => 32-byte B256
    #[inline]
    fn blake3_hash_64(left: &[u8], right: &[u8]) -> B256 {
        let mut hasher = blake3::Hasher::new();
        hasher.update(left);
        hasher.update(right);
        B256::from(hasher.finalize().as_bytes())
    }
}
