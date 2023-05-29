pub struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();

        let mut res = vec![];
        combination_sum(&mut res, &candidates, &mut vec![], 0, target);
        res
    }
}

fn combination_sum(
    res: &mut Vec<Vec<i32>>,
    candidates: &Vec<i32>,
    current: &mut Vec<i32>,
    start: usize,
    target: i32,
) {
    if target == 0 {
        res.push(current.clone());
        return;
    }

    for i in start..candidates.len() {
        if i > start && candidates[i] == candidates[i - 1] {
            continue;
        }

        if candidates[i] <= target {
            current.push(candidates[i]);
            combination_sum(res, candidates, current, i + 1, target - candidates[i]);
            current.pop();
        }
    }
}

#[cfg(test)]
mod forty_test {
    use super::*;

    #[test]
    fn test_forty() {
        assert_eq!(
            // [1,1,2,5,6,7,10]
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8,),
            vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]]
        );
    }
}
