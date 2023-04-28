pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = (&s as &str).trim().as_bytes();

        let first = s[0];
        let iter = s.into_iter();

        let is_positive = first == b'+' || first != b'-';

        let num_str = iter
            .skip_while(|&num| *num == b'0' || *num == b'+' || *num == b'-')
            .take_while(|&num| *num >= b'0' && *num <= b'9')
            // .inspect(|code| println!("code = {code}"))
            .map(|code| *code - 48)
            .collect::<Vec<_>>();

        println!("num_str = {num_str:?}");

        let mut pow = 0;
        let mut res: i32 = Default::default();
        for n in num_str.iter().rev() {
            if let Some(val) = 10_i32.checked_pow(pow) {
                if let Some(data) = res.checked_add((*n as i32) * val) {
                    res = data;
                    pow += 1;
                    continue;
                };
            }
            if is_positive {
                return i32::MAX;
            } else {
                return i32::MIN;
            }
        }

        print!("is_positive = {is_positive}, res = {res}");

        if is_positive {
            res
        } else {
            -res
        }
    }
}

#[cfg(test)]
mod eight_test {
    use super::*;

    #[test]
    fn test_eight() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
    }
}
