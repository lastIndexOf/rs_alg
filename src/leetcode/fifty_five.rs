pub struct Solution;

impl Solution {
    // pub fn can_jump(nums: Vec<i32>) -> bool {
    //     let mut map = std::collections::HashMap::new();
    //     Self::can_jump_for_vec(&nums[..], &mut map)
    // }

    // fn can_jump_for_vec<'a>(
    //     nums: &'a [i32],
    //     map: &mut std::collections::HashMap<&'a [i32], bool>,
    // ) -> bool {
    //     if nums.len() == 1 {
    //         return true;
    //     }

    //     let idx = 0;
    //     let num = nums.first().unwrap();
    //     for step in 1..=(*num) {
    //         if !map.contains_key(nums) {
    //             let next = idx + step as usize;
    //             if next < nums.len() && Self::can_jump_for_vec(&nums[next..], map) {
    //                 return true;
    //             }
    //         } else {
    //             return false;
    //         }
    //     }

    //     map.insert(nums, false);
    //     false
    // }
    // pub fn can_jump(nums: Vec<i32>) -> bool {
    //     let len = nums.len();
    //     let mut res = vec![false; len];

    //     res[len - 1] = true;

    //     for i in (0..(len - 1)).rev() {
    //         let val = nums[i];

    //         for j in 1..=val {
    //             if i + (j as usize) < len && res[i + j as usize] {
    //                 res[i] = true;
    //             }
    //         }
    //     }

    //     res[0]
    // }
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 {
            return true;
        }

        let mut i = nums.len() - 2;
        let mut distance = 1;

        loop {
            if nums[i] >= distance {
                distance = 1;
            } else {
                distance += 1;
            }

            if i == 0 {
                if distance == 1 {
                    return true;
                }

                return false;
            }

            i -= 1;
        }
    }
}

#[cfg(test)]
mod fifty_five_test {
    use super::*;

    #[test]
    fn test_fifty_five() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
