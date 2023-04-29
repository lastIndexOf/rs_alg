pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let s = (&s as &str).trim().as_bytes();

        let mut iter = s.into_iter();

        let first = match iter.next() {
            Some(val) => val,
            None => return 0,
        };

        let first_is_num = is_num(*first);
        let is_positive = *first == b'+' || first_is_num;
        let is_negative = *first == b'-';

        let num_str = std::iter::once_with(|| {
            if (is_negative || is_positive) && !first_is_num {
                &b'0'
            } else {
                first
            }
        })
        .chain(iter)
        .skip_while(|&num| *num == b'0')
        .take_while(|&num| is_num(*num))
        .map(|code| *code - 48)
        .collect::<Vec<_>>();

        println!("first_is_num = {first_is_num}, num_str = {num_str:?}");

        let mut pow = 0;
        let mut res: i32 = Default::default();
        for n in num_str.iter().rev() {
            if let Some(val) = 10_i32.checked_pow(pow) {
                if let Some(mul) = (*n as i32).checked_mul(val) {
                    if let Some(data) = res.checked_add(mul) {
                        res = data;
                        pow += 1;
                        continue;
                    };
                }
            }
            if is_positive {
                return i32::MAX;
            } else {
                return i32::MIN;
            }
        }

        println!("is_positive = {is_positive}, res = {res}");

        if is_positive {
            res
        } else {
            -res
        }
    }
}

fn is_num(code: u8) -> bool {
    code >= b'0' && code <= b'9'
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
        assert_eq!(Solution::my_atoi("-6147483648".to_string()), -2147483648);
    }
}
