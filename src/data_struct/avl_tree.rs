use std::{
    cmp::{max, Ordering},
    fmt::Display,
    marker::PhantomData,
    ops::Not,
};

#[derive(Debug)]
pub struct AVLTree<T: Ord> {
    root: Option<*mut TreeNode<T>>,
    length: usize,
    _marker: PhantomData<Box<TreeNode<T>>>,
}

#[derive(Debug)]
pub(self) struct TreeNode<T: Ord> {
    value: T,
    length: usize,
    left: Option<*mut TreeNode<T>>,
    right: Option<*mut TreeNode<T>>,
    _marker: PhantomData<Box<TreeNode<T>>>,
}

#[derive(Clone, Copy, Debug)]
enum Side {
    Left,
    Right,
}

impl Not for Side {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Side::Left => Self::Right,
            Side::Right => Self::Left,
        }
    }
}

impl<T: Ord> AVLTree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn contains(&self, value: &T) -> bool {
        if self.root.is_none() {
            return false;
        }

        let mut node = self.root;

        while node.is_some() {
            let node_ptr = node.unwrap();
            unsafe {
                match (*node_ptr).value.cmp(value) {
                    Ordering::Equal => return true,
                    Ordering::Greater => {
                        if (*node_ptr).left.is_none() {
                            return false;
                        } else {
                            node = (*node_ptr).left;
                        }
                    }
                    Ordering::Less => {
                        if (*node_ptr).right.is_none() {
                            return false;
                        } else {
                            node = (*node_ptr).right;
                        }
                    }
                }
            }
        }

        return false;
    }
    pub fn len(&self) -> usize {
        self.length
    }
    pub fn insert(&mut self, value: T) -> bool {
        let inserted = insert(&mut self.root, value);

        if inserted {
            self.length += 1;
        }

        inserted
    }
    pub fn remove(&self) {}
    pub fn node_iter(&self) {}
}

fn insert<T: Ord>(tree: &mut Option<*mut TreeNode<T>>, value: T) -> bool {
    if tree.is_none() {
        let new_node = TreeNode::new(value);
        *tree = Some(Box::into_raw(Box::new(new_node)));
        return true;
    }

    let node = tree.unwrap();
    let inserted = unsafe {
        match (*node).value.cmp(&value) {
            Ordering::Equal => return false,
            Ordering::Greater => insert(&mut (*node).left, value),
            Ordering::Less => insert(&mut (*node).right, value),
        }
    };

    if inserted {
        unsafe {
            (*node).rebalance();
        };
    }

    inserted
}

impl<T: Ord> Default for AVLTree<T> {
    fn default() -> Self {
        Self {
            root: None,
            length: 0,
            _marker: PhantomData,
        }
    }
}

impl<T: Ord> TreeNode<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            length: 1,
            left: None,
            right: None,
            _marker: PhantomData,
        }
    }

    fn child(&self, side: Side) -> Option<*const TreeNode<T>> {
        match side {
            Side::Left => self.left.map(|x| (x as *const TreeNode<T>)),
            Side::Right => self.right.map(|x| (x as *const TreeNode<T>)),
        }
    }

    fn child_mut(&mut self, side: Side) -> Option<*mut TreeNode<T>> {
        match side {
            Side::Left => self.left,
            Side::Right => self.right,
        }
    }

    fn balance_factor(&self) -> isize {
        let left = self.child(Side::Left).map_or(0, |x| unsafe { (*x).length });
        let right = self
            .child(Side::Right)
            .map_or(0, |x| unsafe { (*x).length });

        if left > right {
            (left - right) as isize
        } else {
            -((right - left) as isize)
        }
    }

    fn update_height(&mut self) {
        self.length = 1 + max(
            self.child_mut(Side::Left)
                .map_or(0, |x| unsafe { (*x).length }),
            self.child_mut(Side::Right)
                .map_or(0, |x| unsafe { (*x).length }),
        )
    }

    fn rotate(&mut self, side: Side) {
        let subtree = self.child_mut(!side).unwrap();
        unsafe {
            match side {
                Side::Left => self.right = (*subtree).left,
                Side::Right => self.left = (*subtree).right,
            }
        };
        self.update_height();

        unsafe { std::mem::swap(self, &mut *subtree) };
        self.update_height();

        unsafe {
            match side {
                Side::Left => (*(self as *mut Self)).left = Some(subtree),
                Side::Right => (*(self as *mut Self)).right = Some(subtree),
            }
        };
        self.update_height();
    }

    fn rebalance(&mut self) {
        self.update_height();

        let side = match self.balance_factor() {
            2 => Side::Left,
            -2 => Side::Right,
            _ => return,
        };

        let subtree = self.child_mut(side).unwrap();

        unsafe {
            if let (Side::Left, -1) | (Side::Right, 1) = (side, (*subtree).balance_factor()) {
                (*subtree).rotate(side);
            }
        }

        self.rotate(!side);
        // if need_balance {}
    }
}

impl<T: Ord + Display> Display for AVLTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = Vec::with_capacity(self.length);

        bfs(&mut result, self.root);

        write!(f, "result = {:?}", result)
    }
}

fn bfs<T: Ord + Display>(result: &mut Vec<String>, node: Option<*mut TreeNode<T>>) {
    if node.is_some() {
        let node_ptr = node.unwrap();

        result.push(format!(
            "{}(height:{})",
            unsafe { (*node_ptr).value.to_string() },
            unsafe { (*node_ptr).length }
        ));
        bfs(result, unsafe { (*node_ptr).left });
        bfs(result, unsafe { (*node_ptr).right });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Returns `true` if all nodes in the tree are balanced.
    // fn is_balanced<T: Ord>(tree: &AVLTree<T>) -> bool {
    //     tree.node_iter()
    //         .all(|n| (-1..=1).contains(&n.balance_factor()))
    // }

    // #[test]
    // fn len() {
    //     let tree: AVLTree<_> = (1..4).collect();
    //     assert_eq!(tree.len(), 3);
    // }

    // #[test]
    // fn contains() {
    //     let tree: AVLTree<_> = (1..4).collect();
    //     assert!(tree.contains(&1));
    //     assert!(!tree.contains(&4));
    // }

    #[test]
    fn insert() {
        let mut tree = AVLTree::new();
        // First insert succeeds
        assert!(tree.insert(1));
        // Second insert fails
        assert!(!tree.insert(1));
    }

    #[test]
    fn insert_with_num() {
        let mut tree = AVLTree::new();
        tree.insert(1);
        tree.insert(2);
        tree.insert(3);
        tree.insert(4);
        tree.insert(5);
        tree.insert(6);

        println!("tree is {}", tree);
    }

    // #[test]
    // fn remove() {
    //     let mut tree: AVLTree<_> = (1..8).collect();
    //     // First remove succeeds
    //     assert!(tree.remove(&4));
    //     // Second remove fails
    //     assert!(!tree.remove(&4));
    // }

    // #[test]
    // fn sorted() {
    //     let tree: AVLTree<_> = (1..8).rev().collect();
    //     assert!((1..8).eq(tree.iter().copied()));
    // }

    // #[test]
    // fn balanced() {
    //     let mut tree: AVLTree<_> = (1..8).collect();
    //     assert!(is_balanced(&tree));
    //     for x in 1..8 {
    //         tree.remove(&x);
    //         assert!(is_balanced(&tree));
    //     }
    // }
}
