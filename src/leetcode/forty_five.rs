pub struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return 0;
        }

        if nums.len() == 2 {
            return 1;
        }

        let len = nums.len();
        let mut res = vec![1; nums.len()];

        res[len - 1] = 0;
        res[len - 2] = 1;

        for i in (0..(len - 2)).rev() {
            let mut min = res[i + 1];

            if nums[i] >= (len - i - 1) as i32 {
                min = 0;
            } else {
                for j in 1..(nums[i] as usize) {
                    if min > res[i + j + 1] {
                        min = res[i + j + 1]
                    }
                }
            }

            res[i] = 1 + min;
        }

        res[0]
    }
}

#[cfg(test)]
mod forty_five_test {
    use super::*;

    #[test]
    fn test_forty_five() {
        assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
        assert_eq!(Solution::jump(vec![3, 2, 1]), 1);
    }
}
