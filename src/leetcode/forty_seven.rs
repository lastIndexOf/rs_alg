pub struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        Self::do_permute(&nums, &mut res, &mut vec![]);
        res
    }

    pub fn do_permute(nums: &Vec<i32>, res: &mut Vec<Vec<i32>>, current: &mut Vec<i32>) {
        if nums.is_empty() {
            res.push(current.clone());
            return;
        }

        let mut map = std::collections::HashMap::new();
        for i in 0..nums.len() {
            if map.contains_key(&nums[i]) {
                continue;
            }
            map.insert(nums[i], true);
            current.push(nums[i]);
            let mut nums = nums.clone();
            nums.swap_remove(i);
            Self::do_permute(&nums, res, current);
            current.pop();
        }
    }
}

#[cfg(test)]
mod forty_seven_test {
    use super::*;

    #[test]
    fn test_forty_seven() {
        assert_eq!(
            {
                let mut a = Solution::permute_unique(vec![1, 1, 2]);
                a.sort();
                a
            },
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
        assert_eq!(
            {
                let mut a = Solution::permute_unique(vec![1, 2, 3]);
                a.sort();
                a
            },
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ]
        );
    }
}
