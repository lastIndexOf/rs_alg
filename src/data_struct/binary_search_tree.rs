use std::cmp::Ordering;

#[derive(Debug)]
pub struct BinarySearchTree<T: Ord> {
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn new() -> Self {
        BinarySearchTree::default()
    }

    pub fn insert(&mut self, value: T) {
        match self.value.as_ref() {
            Some(val) => match val.cmp(&value) {
                Ordering::Greater => match self.left.as_mut() {
                    Some(left) => left.insert(value),
                    None => {
                        let mut left_tree = Self::default();
                        left_tree.value = Some(value);
                        self.left = Some(Box::new(left_tree));
                    }
                },
                Ordering::Less => match self.right.as_mut() {
                    Some(right) => right.insert(value),
                    None => {
                        let mut right_tree = Self::default();
                        right_tree.value = Some(value);
                        self.right = Some(Box::new(right_tree));
                    }
                },
                _ => {}
            },
            None => self.value = Some(value),
        }
    }

    pub fn search(&self, value: T) -> bool {
        if let Some(val) = self.value.as_ref() {
            match val.cmp(&value) {
                Ordering::Equal => return true,
                Ordering::Greater => match self.left.as_ref() {
                    Some(left) => return left.as_ref().search(value),
                    None => return false,
                },
                Ordering::Less => match self.right.as_ref() {
                    Some(right) => return right.as_ref().search(value),
                    None => return false,
                },
            }
        }

        false
    }

    pub fn maximum(&self) -> Option<&T> {
        if let Some(val) = self.value.as_ref() {
            if let Some(right) = self.right.as_ref() {
                return right.maximum();
            }

            return Some(val);
        }

        None
    }

    pub fn minimum(&self) -> Option<&T> {
        if let Some(val) = self.value.as_ref() {
            if let Some(left) = self.left.as_ref() {
                return left.minimum();
            }

            return Some(val);
        }

        None
    }

    pub fn iter(&self) -> BinarySearchTreeIter<T> {
        BinarySearchTreeIter::new(self)
    }
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self {
            value: None,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct BinarySearchTreeIter<'a, T: Ord> {
    stack: Vec<&'a T>,
}
impl<'a, T: Ord> BinarySearchTreeIter<'a, T> {
    fn new(tree: &'a BinarySearchTree<T>) -> Self {
        let stack = Self::init_iter_stack(tree);
        Self { stack }
    }

    fn init_iter_stack(tree: &'a BinarySearchTree<T>) -> Vec<&'a T> {
        let mut stack = vec![];
        Self::iter_stack(&mut stack, tree);
        stack
    }

    fn iter_stack(stack: &mut Vec<&'a T>, tree: &'a BinarySearchTree<T>) {
        if tree.left.is_none() {
            stack.push(tree.value.as_ref().unwrap());
            return;
        }

        Self::iter_stack(stack, tree.left.as_ref().unwrap());
        stack.push(tree.value.as_ref().unwrap());
        if let Some(right) = tree.right.as_ref() {
            Self::iter_stack(stack, right);
        }
    }
}
impl<'a, T: Ord> Iterator for BinarySearchTreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            return None;
        }

        Some(self.stack.remove(0))
    }
}

#[cfg(test)]
mod test {
    use super::BinarySearchTree;

    fn prequel_memes_tree() -> BinarySearchTree<&'static str> {
        let mut tree = BinarySearchTree::new();
        tree.insert("hello there");
        tree.insert("general kenobi");
        tree.insert("you are a bold one");
        tree.insert("kill him");
        tree.insert("back away...I will deal with this jedi slime myself");
        tree.insert("your move");
        tree.insert("you fool");

        tree
    }

    #[test]
    fn test_search() {
        let tree = prequel_memes_tree();
        assert!(tree.search(&"hello there"));
        assert!(tree.search(&"you are a bold one"));
        assert!(tree.search(&"general kenobi"));
        assert!(tree.search(&"you fool"));
        assert!(tree.search(&"kill him"));
        assert!(
            !tree.search(&"but i was going to tosche station to pick up some power converters",)
        );
        assert!(!tree.search(&"only a sith deals in absolutes"));
        assert!(!tree.search(&"you underestimate my power"));
    }

    #[test]
    fn test_maximum_and_minimum() {
        let tree = prequel_memes_tree();
        assert_eq!(*tree.maximum().unwrap(), "your move");
        assert_eq!(
            *tree.minimum().unwrap(),
            "back away...I will deal with this jedi slime myself"
        );
        let mut tree2: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree2.maximum().is_none());
        assert!(tree2.minimum().is_none());
        tree2.insert(0);
        assert_eq!(*tree2.minimum().unwrap(), 0);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(-5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 0);
        tree2.insert(5);
        assert_eq!(*tree2.minimum().unwrap(), -5);
        assert_eq!(*tree2.maximum().unwrap(), 5);
    }

    // #[test]
    // fn test_floor_and_ceil() {
    //     let tree = prequel_memes_tree();
    //     assert_eq!(*tree.floor(&"hello there").unwrap(), "hello there");
    //     assert_eq!(
    //         *tree
    //             .floor(&"these are not the droids you're looking for")
    //             .unwrap(),
    //         "kill him"
    //     );
    //     assert!(tree.floor(&"another death star").is_none());
    //     assert_eq!(*tree.floor(&"you fool").unwrap(), "you fool");
    //     assert_eq!(
    //         *tree.floor(&"but i was going to tasche station").unwrap(),
    //         "back away...I will deal with this jedi slime myself"
    //     );
    //     assert_eq!(
    //         *tree.floor(&"you underestimate my power").unwrap(),
    //         "you fool"
    //     );
    //     assert_eq!(*tree.floor(&"your new empire").unwrap(), "your move");
    //     assert_eq!(*tree.ceil(&"hello there").unwrap(), "hello there");
    //     assert_eq!(
    //         *tree
    //             .ceil(&"these are not the droids you're looking for")
    //             .unwrap(),
    //         "you are a bold one"
    //     );
    //     assert_eq!(
    //         *tree.ceil(&"another death star").unwrap(),
    //         "back away...I will deal with this jedi slime myself"
    //     );
    //     assert_eq!(*tree.ceil(&"you fool").unwrap(), "you fool");
    //     assert_eq!(
    //         *tree.ceil(&"but i was going to tasche station").unwrap(),
    //         "general kenobi"
    //     );
    //     assert_eq!(
    //         *tree.ceil(&"you underestimate my power").unwrap(),
    //         "your move"
    //     );
    //     assert!(tree.ceil(&"your new empire").is_none());
    // }

    #[test]
    fn test_iterator() {
        let tree = prequel_memes_tree();
        let mut iter = tree.iter();
        dbg!(&iter);
        assert_eq!(
            iter.next().unwrap(),
            &"back away...I will deal with this jedi slime myself"
        );
        assert_eq!(iter.next().unwrap(), &"general kenobi");
        assert_eq!(iter.next().unwrap(), &"hello there");
        assert_eq!(iter.next().unwrap(), &"kill him");
        assert_eq!(iter.next().unwrap(), &"you are a bold one");
        assert_eq!(iter.next().unwrap(), &"you fool");
        assert_eq!(iter.next().unwrap(), &"your move");
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }
}
