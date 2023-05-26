pub struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.is_empty() {
            return vec![-1, -1];
        }

        if nums.len() == 1 {
            if nums[0] == target {
                return vec![0, 0];
            }
            return vec![-1, -1];
        }

        let mut i = 0;
        let mut j = nums.len() - 1;

        while i <= j && nums[i] != target {
            i += 1;
        }
        while j > i && nums[j] != target {
            j -= 1;
        }

        if i <= j {
            return vec![i as i32, j as i32];
        }

        vec![-1, -1]
    }
}

#[cfg(test)]
mod thirty_four_test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_thirty_four() {
        assert_eq!(
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6),
            vec![-1, -1]
        );
    }
}
