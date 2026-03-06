use crate::node::StemNode;
use alloy_primitives::B256;

/// A Merkle proof for inclusion/exclusion of a key-value pair
#[derive(Debug, Default, Clone)]
pub struct MerkleProof {
    /// The nodes along the path from root to target
    pub path: Vec<ProofNode>,
}

/// A node in the Merkle proof path.
#[derive(Debug, Clone)]
pub enum ProofNode {
    /// Internal node's sibling hash
    Hash(B256),
    /// Complete stem node for verification
    Stem(Box<StemNode>),
}

impl MerkleProof {
    /// Creates a new empty proof.
    pub fn new() -> Self {
        Self { path: Vec::new() }
    }

    /// Adds a sibling hash to the proof path.
    pub fn add_hash(&mut self, hash: B256) {
        self.path.push(ProofNode::Hash(hash));
    }

    /// Adds a stem node to the proof path.
    pub fn add_stem(&mut self, node: StemNode) {
        self.path.push(ProofNode::Stem(Box::new(node)));
    }
}
