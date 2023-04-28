use std::{collections::HashMap, hash::Hash};

pub struct UnionFind<T: Hash + Eq> {
    payloads: HashMap<T, usize>,
    parents: Vec<usize>,
    sizes: Vec<usize>,
    count: usize,
}

impl<T: Hash + Eq> UnionFind<T> {
    pub fn insert(&mut self, value: T) {
        let key = self.parents.len();
        self.parents.push(key);
        self.sizes.push(1);
        self.payloads.insert(value, key);
        self.count += 1;
    }

    pub fn find(&self, value: &T) -> Option<usize> {
        self.payloads
            .get(&value)
            .copied()
            .map(|key| self.find_by_key(key))
    }

    pub fn find_by_key(&self, key: usize) -> usize {
        if key >= self.parents.len() {
            panic!("{key} 越界了!");
        }

        let mut parent = key;
        while self.parents[parent] != parent {
            parent = self.parents[parent];
        }

        parent
    }

    pub fn union(&mut self, set_a: &T, set_b: &T) -> Option<bool> {
        match (self.find(set_a), self.find(set_b)) {
            (Some(root_a), Some(root_b)) => self.union_by_key(root_a, root_b),
            _ => None,
        }
    }

    pub fn union_by_key(&mut self, key_a: usize, key_b: usize) -> Option<bool> {
        let (root_a, root_b) = (self.find_by_key(key_a), self.find_by_key(key_b));

        if root_a == root_b {
            return Some(false);
        }

        if self.sizes[root_a] < self.sizes[root_b] {
            self.parents[root_a] = root_b;
            self.sizes[root_b] += self.sizes[root_a]
        } else {
            self.parents[root_b] = root_a;
            self.sizes[root_a] += self.sizes[root_b];
        }

        self.count -= 1;
        Some(true)
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl<T: Hash + Eq> Default for UnionFind<T> {
    fn default() -> Self {
        Self {
            payloads: HashMap::new(),
            parents: Vec::new(),
            sizes: Vec::new(),
            count: 0,
        }
    }
}

impl<A: Hash + Eq> FromIterator<A> for UnionFind<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut union_find = UnionFind::default();

        for item in iter {
            union_find.insert(item);
        }

        union_find
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_find() {
        let mut uf = UnionFind::from_iter(0..10);
        assert_eq!(uf.find_by_key(0), 0);
        assert_eq!(uf.find_by_key(1), 1);
        assert_eq!(uf.find_by_key(2), 2);
        assert_eq!(uf.find_by_key(3), 3);
        assert_eq!(uf.find_by_key(4), 4);
        assert_eq!(uf.find_by_key(5), 5);
        assert_eq!(uf.find_by_key(6), 6);
        assert_eq!(uf.find_by_key(7), 7);
        assert_eq!(uf.find_by_key(8), 8);
        assert_eq!(uf.find_by_key(9), 9);

        assert_eq!(Some(true), uf.union(&0, &1));
        assert_eq!(Some(true), uf.union(&1, &2));
        assert_eq!(Some(true), uf.union(&2, &3));
        assert_eq!(Some(true), uf.union(&3, &4));
        assert_eq!(Some(true), uf.union(&4, &5));
        assert_eq!(Some(true), uf.union(&5, &6));
        assert_eq!(Some(true), uf.union(&6, &7));
        assert_eq!(Some(true), uf.union(&7, &8));
        assert_eq!(Some(true), uf.union(&8, &9));
        assert_eq!(Some(false), uf.union(&9, &0));

        assert_eq!(1, uf.count());
    }

    #[test]
    fn test_spanning_tree() {
        // Let's imagine the following topology:
        //  A <-> B
        //  B <-> C
        //  A <-> D
        //  E
        //  F <-> G
        // We have 3 disjoint sets: {A, B, C, D}, {E}, {F, G}
        let mut uf = UnionFind::from_iter(["A", "B", "C", "D", "E", "F", "G"]);
        uf.union(&"A", &"B");
        uf.union(&"B", &"C");
        uf.union(&"A", &"D");
        uf.union(&"F", &"G");
        assert_eq!(3, uf.count());
    }
}
