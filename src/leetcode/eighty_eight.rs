pub struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut one_idx = m - 1;
        let mut two_idx = n - 1;
        let mut idx = m + n - 1;

        while idx >= 0 {
            if (two_idx >= 0 && one_idx >= 0 && nums1[one_idx as usize] < nums2[two_idx as usize])
                || one_idx < 0
            {
                nums1[idx as usize] = nums2[two_idx as usize];
                two_idx -= 1;
            } else {
                nums1[idx as usize] = nums1[one_idx as usize];
                one_idx -= 1;
            }

            idx -= 1;
        }
    }
}

#[cfg(test)]
mod eighty_eight_test {
    use super::*;

    #[test]
    fn test_eighty_eight() {
        let mut arr1 = vec![1, 2, 3, 0, 0, 0];
        let mut arr2 = vec![2, 5, 6];
        Solution::merge(&mut arr1, 3, &mut arr2, 3);

        assert_eq!(arr1, [1, 2, 2, 3, 5, 6]);
    }
}
