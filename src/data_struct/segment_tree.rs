use std::ops::Range;

#[derive(Debug)]
pub struct SegmentTree<T: Default + Ord + Copy + Clone> {
    tree: Vec<T>,
    length: usize,
    merge: fn(T, T) -> T,
}

impl<T: Default + Ord + Copy + Clone> SegmentTree<T> {
    pub fn from_vec(vector: &Vec<T>, merge: fn(T, T) -> T) -> Self {
        let length = vector.len();
        let mut tree = vec![Default::default(); 2 * length];

        tree[length..(length * 2)].copy_from_slice(&vector[..length]);

        for index in (1..length).rev() {
            tree[index] = merge(tree[index * 2], tree[index * 2 + 1]);
        }

        Self {
            tree,
            length,
            merge,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn query(&self, range: Range<usize>) -> Option<T> {
        let mut left = range.start + self.length;
        let mut right = std::cmp::min(range.end, self.length) + self.length;

        let mut res = None;

        while left < right {
            if left % 2 == 1 {
                res = match res {
                    Some(val) => Some((self.merge)(val, self.tree[left])),
                    None => Some(self.tree[left]),
                };
                left += 1;
            }

            if right % 2 == 1 {
                right -= 1;
                res = match res {
                    Some(val) => Some((self.merge)(val, self.tree[right])),
                    None => Some(self.tree[right]),
                };
            }

            left /= 2;
            right /= 2;
        }

        res
    }

    pub fn update(&mut self, index: usize, value: T) {
        if index >= self.length {
            eprintln!("overflow");
            std::process::exit(1);
        }

        let mut index = index + self.length;
        self.tree[index] = value;

        index /= 2;
        while index > 0 {
            self.tree[index] = (self.merge)(self.tree[index * 2], self.tree[index * 2 + 1]);
            index /= 2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::{max, min};

    #[test]
    fn test_segments_create() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut min_seg_tree = SegmentTree::from_vec(&vec, min);
        min_seg_tree.update(14, -10);
        println!("{:?}", min_seg_tree);
    }

    #[test]
    fn test_min_segments() {
        let vec = vec![-30, 2, -4, 7, 3, -5, 6, 11, -20, 9, 14, 15, 5, 2, -8];
        let min_seg_tree = SegmentTree::from_vec(&vec, min);
        assert_eq!(Some(-5), min_seg_tree.query(4..7));
        assert_eq!(Some(-30), min_seg_tree.query(0..vec.len()));
        assert_eq!(Some(-30), min_seg_tree.query(0..2));
        assert_eq!(Some(-4), min_seg_tree.query(1..3));
        assert_eq!(Some(-5), min_seg_tree.query(1..7));
    }

    #[test]
    fn test_max_segments() {
        let val_at_6 = 6;
        let vec = vec![1, 2, -4, 7, 3, -5, val_at_6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut max_seg_tree = SegmentTree::from_vec(&vec, max);
        assert_eq!(Some(15), max_seg_tree.query(0..vec.len()));
        let max_4_to_6 = 6;
        assert_eq!(Some(max_4_to_6), max_seg_tree.query(4..7));
        let delta = 2;
        max_seg_tree.update(6, val_at_6 + delta);
        assert_eq!(Some(val_at_6 + delta), max_seg_tree.query(4..7));
    }

    #[test]
    fn test_sum_segments() {
        let val_at_6 = 6;
        let vec = vec![1, 2, -4, 7, 3, -5, val_at_6, 11, -20, 9, 14, 15, 5, 2, -8];
        let mut sum_seg_tree = SegmentTree::from_vec(&vec, |a, b| a + b);
        for (i, val) in vec.iter().enumerate() {
            assert_eq!(Some(*val), sum_seg_tree.query(i..(i + 1)));
        }
        let sum_4_to_6 = sum_seg_tree.query(4..7);
        assert_eq!(Some(4), sum_4_to_6);
        let delta = 3;
        sum_seg_tree.update(6, val_at_6 + delta);
        assert_eq!(
            sum_4_to_6.unwrap() + delta,
            sum_seg_tree.query(4..7).unwrap()
        );
    }
}
