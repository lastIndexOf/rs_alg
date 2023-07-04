pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut map = std::collections::HashMap::from([(0, 0), (1, 0), (2, 0)]);

        for num in nums.iter() {
            map.entry(*num).and_modify(|item| *item += 1);
        }

        let res = std::iter::repeat(0)
            .take(map[&0_i32])
            .chain(std::iter::repeat(1).take(map[&1_i32]))
            .chain(std::iter::repeat(2).take(map[&2_i32]))
            .collect::<Vec<_>>();

        *nums = res;
    }
}

#[cfg(test)]
mod seventy_five_test {
    use super::*;

    #[test]
    fn test_seventy_five() {
        let mut v = vec![2, 0, 2, 1, 1, 0];
        Solution::sort_colors(&mut v);
        assert_eq!(v, [0, 0, 1, 1, 2, 2]);

        let mut v = vec![2, 0, 1];
        Solution::sort_colors(&mut v);
        assert_eq!(v, [0, 1, 2]);
    }
}
