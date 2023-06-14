pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = Vec::<i32>::with_capacity(nums.len());
        let mut nums = nums.into_iter();

        res.push(nums.next().unwrap());

        for (idx, num) in nums.enumerate() {
            res.push(num + std::cmp::max(res[idx], 0));
        }

        res.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod fifty_three_test {
    use super::*;

    #[test]
    fn test_fifty_three() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(Solution::max_sub_array(vec![-2, -1]), -1);
        assert_eq!(Solution::max_sub_array(vec![-1, 0]), 0);
    }
}
