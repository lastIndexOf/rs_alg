pub struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        let n = nums.len();
        for k in 0..=n {
            Self::find_subsets(&mut res, &mut vec![], &nums, 0, k);
        }
        res
    }

    fn find_subsets(
        res: &mut Vec<Vec<i32>>,
        current: &mut Vec<i32>,
        raw: &Vec<i32>,
        start: usize,
        k: usize,
    ) {
        if k == 0 {
            res.push(current.clone());
            return;
        }

        for idx in start..raw.len() {
            current.push(raw[idx]);
            Self::find_subsets(res, current, raw, idx + 1, k - 1);
            current.pop();
        }
    }
}

#[cfg(test)]
mod seventy_eight_test {
    use super::*;

    #[test]
    fn test_seventy_eight() {
        let res = [
            vec![],
            vec![1],
            vec![2],
            vec![3],
            vec![1, 2],
            vec![1, 3],
            vec![2, 3],
            vec![1, 2, 3],
        ];
        assert_eq!(Solution::subsets(vec![1, 2, 3]), res);

        let res = [vec![], vec![0]];
        assert_eq!(Solution::subsets(vec![0]), res);
    }
}
