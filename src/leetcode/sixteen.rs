use std::iter::Sum;

/// 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。
/// 返回这三个数的和。
/// 假定每组输入只存在恰好一个解。

pub struct Solution;

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut res = vec![];

        any_sum(&nums, &mut res, &mut vec![], 0, 3);

        println!("res = {res:?} ");
        res[res
            .iter()
            .map(|item| (item.iter().sum::<i32>() - target).abs())
            .enumerate()
            .min_by_key(|(_, value)| *value)
            .map(|(idx, _)| idx)
            .unwrap()]
        .iter()
        .sum()
    }
}

fn any_sum(
    nums: &Vec<i32>,
    res: &mut Vec<Vec<i32>>,
    current_list: &mut Vec<i32>,
    start: usize,
    count: usize,
) {
    if count == 0 {
        let mut cloned_current_list = current_list.clone();
        cloned_current_list.sort();
        res.push(cloned_current_list);
        return;
    }

    for idx in start..=(nums.len() - count) {
        current_list.push(nums[idx]);
        any_sum(nums, res, current_list, idx + 1, count - 1);
        current_list.pop();
    }
}

#[cfg(test)]
mod sixteen_test {
    use super::*;

    #[test]
    fn test_sixteen() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1,), 2);
        assert_eq!(Solution::three_sum_closest(vec![0, 0, 0], 1,), 0);
    }
}
