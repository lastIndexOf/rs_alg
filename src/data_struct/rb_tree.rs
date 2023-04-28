// use std::marker::PhantomData;

// pub enum RBNodeColor {
//     Red,
//     Black,
// }

// pub struct RBTree<T, K> {
//     root: *mut RBNode<T, K>,
// }

// pub struct RBNode<T, K> {
//     key: T,
//     value: K,
//     color: RBNodeColor,
//     left: *mut RBNode<T, K>,
//     right: *mut RBNode<T, K>,
//     _marker: PhantomData<Box<RBNode<T, K>>>,
// }

// impl<'a, T, K> RBTree<T, K> {
//     pub fn new() -> Self {
//         Self {
//             root: std::ptr::null_mut(),
//         }
//     }
//     pub fn insert(&mut self, key: T, value: K) {
//         if self.root.is_null() {
//             let root = Box::new(RBNode::new(key, value));
//             self.root = Box::into_raw(root);
//         } else {
//         }
//     }

//     pub fn delete(&self, t: &T) {}

//     pub fn find(&'a self, t: &T) -> Option<&'a K> {
//         None
//     }

//     pub fn iter(&self) -> RBTreeIter {
//         RBTreeIter::new()
//     }
// }

// impl<T, K> RBNode<T, K> {
//     pub fn new(key: T, value: K) -> Self {
//         Self {
//             key,
//             value,
//             color: RBNodeColor::Black,
//             left: std::ptr::null_mut(),
//             right: std::ptr::null_mut(),
//             _marker: PhantomData,
//         }
//     }
// }

// pub struct RBTreeIter {}

// impl RBTreeIter {
//     pub fn new() -> Self {
//         Self {}
//     }
// }
// impl Iterator for RBTreeIter {
//     type Item = ();

//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn find() {
//         let mut tree = RBTree::<usize, char>::new();
//         for (k, v) in String::from("hello, world!").chars().enumerate() {
//             tree.insert(k, v);
//         }
//         assert_eq!(*tree.find(&3).unwrap_or(&'*'), 'l');
//         assert_eq!(*tree.find(&6).unwrap_or(&'*'), ' ');
//         assert_eq!(*tree.find(&8).unwrap_or(&'*'), 'o');
//         assert_eq!(*tree.find(&12).unwrap_or(&'*'), '!');
//     }

//     #[test]
//     fn insert() {
//         let mut tree = RBTree::<usize, char>::new();
//         for (k, v) in String::from("hello, world!").chars().enumerate() {
//             tree.insert(k, v);
//         }
//         let s: String = tree.iter().map(|x| x.value).collect();
//         assert_eq!(s, "hello, world!");
//     }

//     #[test]
//     fn delete() {
//         let mut tree = RBTree::<usize, char>::new();
//         for (k, v) in String::from("hello, world!").chars().enumerate() {
//             tree.insert(k, v);
//         }
//         tree.delete(&1);
//         tree.delete(&3);
//         tree.delete(&5);
//         tree.delete(&7);
//         tree.delete(&11);
//         let s: String = tree.iter().map(|x| x.value).collect();
//         assert_eq!(s, "hlo orl!");
//     }
// }
