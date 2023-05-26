pub struct Solution;

impl Solution {
    // pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    //     match nums.binary_search(&target) {
    //         Ok(res) => return res as i32,
    //         Err(res) => return res as i32,
    //     }
    // }
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        match nums.binary_search(&target) {
            Ok(res) => return res as i32,
            Err(res) => return res as i32,
        }
    }
}

#[cfg(test)]
mod thirty_five_test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_thirty_five() {
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
