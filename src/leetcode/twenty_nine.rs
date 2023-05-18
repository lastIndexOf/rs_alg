use std::println;

pub struct Solution;

impl Solution {
    // pub fn divide(dividend: i32, divisor: i32) -> i32 {
    //     if dividend == i32::MIN && divisor == -1 {
    //         return i32::MAX;
    //     }

    //     let mut count = 0_i64;
    //     let mut res = 0_i64;

    //     let dividend_abs = (dividend as i64).abs();
    //     let divisor_abs = (divisor as i64).abs();

    //     while res <= dividend_abs {
    //         res += divisor_abs;
    //         count += 1;
    //     }

    //     println!("count = {count}");

    //     if dividend ^ divisor < 0 {
    //         let res = -count + 1;
    //         if res < i32::MIN as i64 {
    //             i32::MIN
    //         } else {
    //             res as i32
    //         }
    //     } else {
    //         let res = count - 1;
    //         if res > i32::MAX as i64 {
    //             i32::MAX
    //         } else {
    //             res as i32
    //         }
    //     }
    // }
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }
        let mut res: i64 = 0;
        let m = (dividend as i64).abs();
        let n = (divisor as i64).abs();
        if m < n {
            return 0;
        }
        let mut t = n;
        let mut p: i64 = 1;
        while m > (t << 1) {
            t <<= 1;
            p <<= 1;
        }
        res += p + (Self::divide((m - t) as i32, n as i32)) as i64;

        if (dividend < 0) ^ (divisor < 0) {
            res = -res;
        }
        if res > i32::MAX as i64 {
            return i32::MAX;
        }
        res as i32
    }
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
