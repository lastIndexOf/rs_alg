
#[cfg(test)]
mod tests {
    use super::Treap;

    /// Returns `true` if all nodes in the tree are valid.
    fn is_valid<T: Ord>(tree: &Treap<T>) -> bool {
        tree.node_iter().all(|n| n.is_valid())
    }

    #[test]
    fn len() {
        let tree: Treap<_> = (1..4).collect();
        assert_eq!(tree.len(), 3);
    }

    #[test]
    fn contains() {
        let tree: Treap<_> = (1..4).collect();
        assert!(tree.contains(&1));
        assert!(!tree.contains(&4));
    }

    #[test]
    fn insert() {
        let mut tree = Treap::new();
        // First insert succeeds
        assert!(tree.insert(1));
        // Second insert fails
        assert!(!tree.insert(1));
    }

    #[test]
    fn remove() {
        let mut tree: Treap<_> = (1..8).collect();
        // First remove succeeds
        assert!(tree.remove(&4));
        // Second remove fails
        assert!(!tree.remove(&4));
    }

    #[test]
    fn sorted() {
        let tree: Treap<_> = (1..8).rev().collect();
        assert!((1..8).eq(tree.iter().copied()));
    }

    #[test]
    fn valid() {
        let mut tree: Treap<_> = (1..8).collect();
        assert!(is_valid(&tree));
        for x in 1..8 {
            tree.remove(&x);
            assert!(is_valid(&tree));
        }
    }
}
