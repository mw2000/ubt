use alloy_primitives::B256;

/// The node type in our binary tree.
#[derive(Clone, Debug)]
pub enum Node {
    /// Empty sub-tree.
    Empty,
    /// Internal node (branches left and right).
    Internal(Box<InternalNode>),
    /// Stem node: holds a "stem" (31 bytes) plus 256 leaves of 32-byte values.
    Stem(Box<StemNode>),
}

/// An internal node has two children, each is a `Node`.
#[derive(Clone, Debug)]
pub struct InternalNode {
    pub left: Node,
    pub right: Node,
}

/// A stem node contains a 31-byte "stem" and 256 values.
/// Each value is optional (None = empty).
#[derive(Clone, Debug)]
pub struct StemNode {
    pub stem: [u8; 31],
    pub values: [Option<B256>; 256],
}
