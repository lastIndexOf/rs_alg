pub struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let len = nums.len();

        for i in 0..len {
            while nums[i] > 0 && nums[i] < len as i32 {
                let pos = nums[i] - 1;

                if nums[pos as usize] == nums[i] {
                    break;
                }

                nums.swap(i, pos as usize);
            }
        }

        for i in 0..len {
            if nums[i] != (i + 1) as i32 {
                return (i + 1) as i32;
            }
        }

        (len + 1) as i32
    }
}

#[cfg(test)]
mod forty_one_test {
    use super::*;

    #[test]
    fn test_forty_one() {
        assert_eq!(Solution::first_missing_positive(vec![1, 2, 0]), 3);
        assert_eq!(Solution::first_missing_positive(vec![3, 4, -1, 1]), 2);
        assert_eq!(Solution::first_missing_positive(vec![7, 8, 9, 11, 12]), 1);
    }
}
