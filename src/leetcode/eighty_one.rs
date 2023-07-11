pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut left = 0;
        let mut right = nums.len() - 1;

        if left == right && nums[left] == target {
            return true;
        }

        while left <= right {
            while left < right && nums[left] == nums[left + 1] {
                left += 1;
            }
            while right > left && nums[right] == nums[right - 1] {
                right -= 1;
            }

            let middle = (right + left) / 2;

            if nums[middle] == target {
                return true;
            }

            if nums[middle] >= nums[left] {
                if target >= nums[left] && target < nums[middle] {
                    right = middle - 1;
                } else {
                    left = middle + 1;
                }
            } else {
                if target > nums[middle] && target <= nums[right] {
                    left = middle + 1;
                } else {
                    right = middle - 1;
                }
            }
        }

        false
    }
}
#[cfg(test)]
mod eighty_one_test {
    use super::*;

    #[test]
    fn test_eighty_one() {
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0), true);
        assert_eq!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3), false);
        assert_eq!(Solution::search(vec![1], 0), false);
        assert_eq!(Solution::search(vec![1, 0, 1, 1, 1], 0), true);
        assert_eq!(
            Solution::search(
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
                2,
            ),
            true
        );
        assert_eq!(Solution::search(vec![1], 1,), true);
        assert_eq!(Solution::search(vec![1, 1], 0,), false);
        assert_eq!(Solution::search(vec![1, 3], 3,), true);
        assert_eq!(Solution::search(vec![1, 3, 5], 1,), true);
    }
}
