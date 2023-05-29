pub struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        let mut res = vec![];
        
        candidates.sort();
        combination_sum(&mut res, &mut vec![], &candidates, target);

        res.into_iter()
            .map(|mut item| {
                item.sort();
                item
            })
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    }
}

fn combination_sum(
    res: &mut Vec<Vec<i32>>,
    current: &mut Vec<i32>,
    candidates: &Vec<i32>,
    target: i32,
) {
    for candidate in candidates {
        match target.cmp(candidate) {
            std::cmp::Ordering::Equal => {
                current.push(target);
                res.push(current.clone());
                current.pop();
            }
            std::cmp::Ordering::Greater => {
                current.push(*candidate);
                combination_sum(res, current, candidates, target - candidate);
                current.pop();
            }
            std::cmp::Ordering::Less => {}
        }
    }
}

#[cfg(test)]
mod thirty_nine_test {
    // use std::assert_eq;

    // use super::*;

    #[test]
    fn test_thirty_nine() {
        // assert_eq!(
        //     Solution::combination_sum(vec![2, 3, 5], 8),
        //     vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]]
        // );
        // assert_eq!(
        //     Solution::combination_sum(vec![2, 3, 6, 7], 7),
        //     vec![vec![2, 2, 3], vec![7]]
        // );
        // assert_eq!(
        //     Solution::combination_sum(vec![8, 7, 4, 3], 11),
        //     vec![vec![8, 3], vec![7, 4], vec![4, 4, 3]]
        // );
    }
}
