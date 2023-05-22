pub struct Solution;

impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>) {
        for i in (1..nums.len()).rev() {
            for j in (0..i).rev() {
                if nums[i] > nums[j] {
                    nums.swap(i, j);
                    nums[(j + 1)..].sort_unstable();
                    return;
                }
            }
        }

        nums.sort_unstable();
    }
}

#[cfg(test)]
mod thirty_one_test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_thirty_one() {
        let mut arr = vec![1, 2, 3];
        Solution::next_permutation(&mut arr);
        assert_eq!(arr, vec![1, 3, 2]);

        let mut arr = vec![3, 2, 1];
        Solution::next_permutation(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);

        let mut arr = vec![1, 1, 5];
        Solution::next_permutation(&mut arr);
        assert_eq!(arr, vec![1, 5, 1]);

        let mut arr = vec![1, 3, 2];
        Solution::next_permutation(&mut arr);
        assert_eq!(arr, vec![2, 1, 3]);

        let mut arr = vec![4, 2, 0, 2, 3, 2, 0];
        Solution::next_permutation(&mut arr);
        assert_eq!(arr, vec![4, 2, 0, 3, 0, 2, 2]);
    }
}
