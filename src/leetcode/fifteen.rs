pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];

        any_num(&nums, &mut res, &mut vec![], 0, 0, 3);

        res.into_iter()
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    }
}

fn any_num(
    nums: &Vec<i32>,
    res: &mut Vec<Vec<i32>>,
    current_vec: &mut Vec<i32>,
    target: i32,
    start: usize,
    count: usize,
) {
    if count == 0 && target == 0 {
        let mut current_vec = current_vec.clone();
        current_vec.sort();
        res.push(current_vec);
        return;
    }

    if count == 0 {
        return;
    }

    for idx in start..=(nums.len() - count) {
        current_vec.push(nums[idx]);
        any_num(
            nums,
            res,
            current_vec,
            target - nums[idx],
            idx + 1,
            count - 1,
        );
        current_vec.pop();
    }
}

#[cfg(test)]
mod fifteen_test {
    use super::*;

    #[test]
    fn test_fifteen() {
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![[-1, -1, 2], [-1, 0, 1]]
        );
        assert_eq!(Solution::three_sum(vec![0, 1, 1]), Vec::<Vec<i32>>::new());
    }
}
