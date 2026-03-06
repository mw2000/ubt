pub(super) mod test_utils {
    use crate::{BinaryTree, Node};

    /// Verifies basic tree invariants:
    /// - Stem nodes have correct structure
    /// - Internal nodes have at least one non-empty child
    /// - Tree depth doesn't exceed 248
    pub(crate) fn verify_tree_invariants(tree: &BinaryTree) -> bool {
        verify_node_invariants(&tree.root, 0)
    }

    fn verify_node_invariants(node: &Node, depth: usize) -> bool {
        if depth >= 248 {
            return false;
        }

        match node {
            Node::Empty => true,
            Node::Stem(stem_node) => {
                // Verify at least one value exists
                stem_node.values.iter().any(std::option::Option::is_some)
            }
            Node::Internal(internal) => {
                // Verify at least one child is non-empty
                !matches!(internal.left, Node::Empty)
                    || !matches!(internal.right, Node::Empty)
                        && verify_node_invariants(&internal.left, depth + 1)
                        && verify_node_invariants(&internal.right, depth + 1)
            }
        }
    }
}
