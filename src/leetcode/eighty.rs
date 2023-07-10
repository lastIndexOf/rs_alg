pub struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 1;
        }

        let mut len = nums.len();
        let mut left = 0;
        let mut right = 1;
        let mut repeat = false;

        while right < len && nums[left] <= nums[right] {
            if nums[left] == nums[right] {
                if repeat {
                    nums.remove(right);
                    len = nums.len();
                } else {
                    left += 1;
                    right += 1;
                    repeat = true;
                }
            } else {
                repeat = false;
                left += 1;
                right += 1;
            }
        }

        right as i32
    }
}

#[cfg(test)]
mod eighty_test {
    use super::*;

    #[test]
    fn test_eighty() {
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 1, 2, 2, 3]), 5);
        assert_eq!(
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3]),
            7
        );
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1]), 2);
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 1]), 2);
        assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 1, 1]), 2);
    }
}
