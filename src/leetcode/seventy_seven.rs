pub struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        Self::do_combine(&mut res, &mut vec![], 1, n, k);
        res
    }

    fn do_combine(res: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, start: i32, n: i32, k: i32) {
        if k == 0 {
            res.push(current.clone());
            return;
        }

        for num in start..=n {
            current.push(num);
            Self::do_combine(res, current, num + 1, n, k - 1);
            current.pop();
        }
    }
}

#[cfg(test)]
mod seventy_seven_test {
    use super::*;

    #[test]
    fn test_seventy_seven() {
        let mut res = [[2, 4], [3, 4], [2, 3], [1, 2], [1, 3], [1, 4]];
        res.sort();
        assert_eq!(Solution::combine(4, 2), res);
    }
}
