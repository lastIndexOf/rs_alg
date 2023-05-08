use std::{collections::HashMap, hash::Hash};

pub struct Trie<K: Hash + Eq, V: Default> {
    root: TrieNode<K, V>,
    len: usize,
}

impl<K: Hash + Eq, V: Default> Trie<K, V> {
    pub fn new() -> Self {
        Trie {
            root: Default::default(),
            len: 0,
        }
    }

    pub fn insert(&mut self, keys: impl IntoIterator<Item = K>, value: V) {
        let mut node = &mut self.root;
        for key in keys.into_iter() {
            node = node.next.entry(key).or_insert(Default::default());
        }
        node.value = value;
        self.len += 1;
    }

    pub fn get(&self, keys: impl IntoIterator<Item = K>) -> Option<&V> {
        if self.is_empty() {
            return None;
        }

        let mut node = &self.root;

        for key in keys.into_iter() {
            match node.next.get(&key) {
                Some(key_node) => node = key_node,
                _ => return None,
            };
        }

        Some(&node.value)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

struct TrieNode<K: Hash + Eq, V: Default> {
    value: V,
    next: HashMap<K, TrieNode<K, V>>,
}

impl<K: Hash + Eq, V: Default> Default for TrieNode<K, V> {
    fn default() -> Self {
        Self {
            value: Default::default(),
            next: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_insertion() {
        let mut trie = Trie::new();
        assert_eq!(trie.get("".chars()), None);

        trie.insert("foo".chars(), 1);
        trie.insert("foobar".chars(), 2);

        let mut trie = Trie::new();
        assert_eq!(trie.get(vec![1, 2, 3]), None);

        trie.insert(vec![1, 2, 3], 1);
        trie.insert(vec![3, 4, 5], 2);
    }

    #[test]
    fn test_get() {
        let mut trie = Trie::new();
        trie.insert("foo".chars(), 1);
        trie.insert("foobar".chars(), 2);
        trie.insert("bar".chars(), 3);
        trie.insert("baz".chars(), 4);

        assert_eq!(trie.get("foo".chars()), Some(&1));
        assert_eq!(trie.get("food".chars()), None);

        let mut trie = Trie::new();
        trie.insert(vec![1, 2, 3, 4], 1);
        trie.insert(vec![42], 2);
        trie.insert(vec![42, 6, 1000], 3);
        trie.insert(vec![1, 2, 4, 16, 32], 4);

        assert_eq!(trie.get(vec![42, 6, 1000]), Some(&3));
        assert_eq!(trie.get(vec![43, 44, 45]), None);
    }
}
