pub struct Solution;

impl Solution {
    // 双指针法
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = 0;

        for i in 0..nums.len() {
            if nums[i] != val {
                nums[idx] = nums[i];
                idx += 1;
            }
        }

        idx as i32
    }
}

#[cfg(test)]
mod twenty_seven_test {
    // use super::*;

    #[test]
    fn test_twenty_seven() {}
}
