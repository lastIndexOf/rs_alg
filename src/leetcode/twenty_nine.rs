pub struct Solution;

impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        let is_positive = dividend ^ divisor > 0;

        let dividend = (dividend as i64).abs();
        let divisor = (divisor as i64).abs();

        if dividend < divisor {
            return 0;
        }

        let mut two_pow = 1;

        while dividend >= divisor * (two_pow << 1) {
            two_pow <<= 1;
        }

        let mut res =
            two_pow + Self::divide((dividend - divisor * two_pow) as i32, divisor as i32) as i64;

        if !is_positive {
            res = -res;
        }
        if res > i32::MAX as i64 {
            return i32::MAX;
        }

        res as i32
    }

    // pub fn divide(dividend: i32, divisor: i32) -> i32 {
    //     if dividend == i32::MIN && divisor == -1 {
    //         return i32::MAX;
    //     }
    //     let mut res: i64 = 0;
    //     let m = (dividend as i64).abs();
    //     let n = (divisor as i64).abs();
    //     if m < n {
    //         return 0;
    //     }
    //     let mut t = n;
    //     let mut p: i64 = 1;
    //     while m > (t << 1) {
    //         t <<= 1;
    //         p <<= 1;
    //     }
    //     res += p + (Self::divide((m - t) as i32, n as i32)) as i64;

    //     if (dividend < 0) ^ (divisor < 0) {
    //         res = -res;
    //     }
    //     if res > i32::MAX as i64 {
    //         return i32::MAX;
    //     }
    //     res as i32
    // }
}

#[cfg(test)]
mod twenty_nine_test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_twenty_nine() {
        assert_eq!(Solution::divide(10, 3), 3);
        assert_eq!(Solution::divide(7, -3), -2);
        assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
        assert_eq!(Solution::divide(-2147483648, 1), -2147483648);
    }
}
