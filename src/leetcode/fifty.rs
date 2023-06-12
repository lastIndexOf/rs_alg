pub struct Solution;

impl Solution {
    // 快速幂
    // 任意正整数 = 2^n0 + 2 ^n1 + ...
    // n = 2^n0
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 {
            return 1_f64;
        }

        let res = Self::my_pow(x, n / 2);

        if n % 2 == 0 {
            return res * res;
        }

        if n > 0 {
            return res * res * x;
        }

        res * res / x
    }
}

#[cfg(test)]
mod fifty_test {
    use super::*;

    #[test]
    fn test_fifty() {
        assert_eq!(Solution::my_pow(2.00000, 10), 1024.00000);
        assert_eq!(Solution::my_pow(2.00000, -2), 0.250000);
        assert_eq!(Solution::my_pow(0.00001, 2147483647), 0_f64);
    }
}
