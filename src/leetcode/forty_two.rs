pub struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        if height.len() == 1 {
            return 0;
        }

        let (idx, _) = height
            .iter()
            .enumerate()
            .max_by_key(|(_, &item)| item)
            .unwrap();

        let mut left = 0;
        let mut right = height.len() - 1;
        let mut res = 0;
        let mut current = height[left];

        while left < idx {
            if current > height[left + 1] {
                res += current - height[left + 1];
            } else {
                current = height[left + 1];
            }
            left += 1;
        }

        current = height[right];
        while right > idx {
            if current > height[right - 1] {
                res += current - height[right - 1];
            } else {
                current = height[right - 1];
            }
            right -= 1;
        }

        res
    }
}

#[cfg(test)]
mod forty_two_test {
    use super::*;

    #[test]
    fn test_forty_two() {
        assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
        assert_eq!(Solution::trap(vec![4, 2, 3]), 1);
        assert_eq!(Solution::trap(vec![5, 4, 1, 2]), 1);
    }
}
