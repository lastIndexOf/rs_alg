pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            if nums[0] != target {
                return -1;
            }
            return 0;
        }

        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut middle = right / 2;

        loop {
            if left == right - 1 {
                break;
            }

            if nums[left] < nums[middle] {
                left = middle;
            } else {
                right = middle;
            }
            middle = (right - left) / 2 + left;
        }

        if target > nums[left] {
            return -1;
        }

        if target < nums[right] {
            return -1;
        }

        if target >= nums[0] {
            match &nums[0..=left].binary_search(&target) {
                Ok(res) => return *res as i32,
                Err(_) => -1,
            };
        }

        if target <= nums[nums.len() - 1] {
            match &nums[left..nums.len()].binary_search(&target) {
                Ok(res) => return (*res + left) as i32,
                Err(_) => -1,
            };
        }

        -1
    }
}

#[cfg(test)]
mod thirty_three_test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_thirty_three() {
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3), -1);
        assert_eq!(Solution::search(vec![1], 0), -1);
        assert_eq!(Solution::search(vec![1, 3], 1), 0);
    }
}
