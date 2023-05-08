pub struct Solution;

impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];

        any_sum(&nums, &mut res, &mut vec![], 0, 4, target);

        res.into_iter()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    }
}

fn any_sum(
    nums: &Vec<i32>,
    res: &mut Vec<Vec<i32>>,
    current_list: &mut Vec<i32>,
    start: usize,
    count: usize,
    target: i32,
) {
    if count == 0 {
        if target == 0 {
            let mut cloned_list = current_list.clone();
            cloned_list.sort();
            res.push(cloned_list);
        }
        return;
    }

    for idx in start..nums.len() {
        current_list.push(nums[idx]);
        any_sum(
            nums,
            res,
            current_list,
            idx + 1,
            count - 1,
            target - nums[idx],
        );
        current_list.pop();
    }
}

#[cfg(test)]
mod eighteen_test {
    use super::*;

    #[test]
    fn test_eighteen() {
        assert_eq!(
            Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0),
            vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]]
        );
        assert_eq!(
            Solution::four_sum(vec![2, 2, 2, 2, 2], 8),
            vec![vec![2, 2, 2, 2]]
        );
    }
}
