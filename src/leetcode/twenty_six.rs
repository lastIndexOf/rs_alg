pub struct Solution;

impl Solution {
    // One
    pub fn remove_duplicates_one(nums: &mut Vec<i32>) -> i32 {
        *nums = nums
            .into_iter()
            .map(|item| *item)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        nums.sort();

        nums.len() as i32
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 0;

        for i in 1..nums.len() {
            if nums[idx] != nums[i] {
                nums[idx + 1] = nums[i];
                idx += 1;
            }
        }

        idx as i32 + 1
    }
}

#[cfg(test)]
mod twenty_six_test {
    // use super::*;

    #[test]
    fn test_twenty_six() {}
}
