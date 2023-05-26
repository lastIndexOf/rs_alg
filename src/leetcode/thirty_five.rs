pub struct Solution;

impl Solution {
    // pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    //     match nums.binary_search(&target) {
    //         Ok(res) => return res as i32,
    //         Err(res) => return res as i32,
    //     }
    // }
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.len() == 1 {
            match target.cmp(&nums[0]) {
                std::cmp::Ordering::Greater => return 1,
                _ => return 0,
            }
        }

        let mut left = 0;
        let mut right = nums.len() - 1;
        let mut middle = right / 2;

        while left != right - 1 {
            match nums[middle].cmp(&target) {
                std::cmp::Ordering::Equal => return middle as i32,
                std::cmp::Ordering::Greater => {
                    right = middle;
                    middle = (right - left) / 2 + left;
                }
                std::cmp::Ordering::Less => {
                    left = middle;
                    middle = (right - left) / 2 + left;
                }
            }
        }

        if target <= nums[left] {
            return left as i32;
        }

        if target > nums[right] {
            return (right + 1) as i32;
        }

        right as i32
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
        assert_eq!(Solution::search_insert(vec![1], 0), 0);
        assert_eq!(Solution::search_insert(vec![1, 3], 1), 0);
    }
}
