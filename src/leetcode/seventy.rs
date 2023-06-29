pub struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut res = Vec::with_capacity(n as usize);

        for step in 0..2 {
            res.push(step + 1);
        }

        for step in 2..n {
            let step = step as usize;
            res.push(res[step - 1] + res[step - 2]);
        }

        res[n as usize - 1]
    }
}

#[cfg(test)]
mod seventy_test {
    use super::*;

    #[test]
    fn test_seventy() {
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(45), 1836311903);
    }
}
