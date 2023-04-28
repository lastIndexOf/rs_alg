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
