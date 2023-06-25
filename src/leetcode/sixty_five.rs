pub struct Solution;

impl Solution {
    pub fn is_number(s: String) -> bool {
        let s = s.as_bytes();

        match s.iter().position(|cr| *cr == b'e' || *cr == b'E') {
            Some(idx) => {
                return (Self::is_decimal(&s[..idx]) || Self::is_integer(&s[..idx]))
                    && Self::is_integer(&s[(idx + 1)..])
            }
            None => return Self::is_decimal(&s[..]) || Self::is_integer(&s[..]),
        };
    }

    fn is_decimal(s: &[u8]) -> bool {
        let mut iter = s.iter();

        if let Some(first) = iter.next() {
            let mut current = if *first == b'+' || *first == b'-' {
                iter.next()
            } else {
                Some(first)
            };

            if let Some(val) = current {
                if *val == b'.' {
                    current = iter.next();

                    if current.is_none() {
                        return false;
                    }

                    while let Some(val) = current {
                        if *val < b'0' || *val > b'9' {
                            return false;
                        }

                        current = iter.next();
                    }

                    return true;
                }

                let mut has_dot = false;
                while let Some(val) = current {
                    if *val < b'0' || *val > b'9' {
                        if *val == b'.' && !has_dot {
                            has_dot = true;
                        } else {
                            return false;
                        }
                    }

                    current = iter.next();
                }

                return has_dot;
            } else {
                return false;
            }
        } else {
            return false;
        };
    }

    fn is_integer(s: &[u8]) -> bool {
        let mut iter = s.iter();

        if let Some(first) = iter.next() {
            let mut current = if *first == b'+' || *first == b'-' {
                iter.next()
            } else {
                Some(first)
            };

            let mut num_num = 0;
            while let Some(val) = current {
                if *val < b'0' || *val > b'9' {
                    return false;
                }

                current = iter.next();
                num_num += 1;
            }

            return num_num > 0;
        } else {
            return false;
        };
    }
}

#[cfg(test)]
mod sixty_five_test {
    use super::*;

    #[test]
    fn test_sixty_five() {
        assert_eq!(Solution::is_number(".".to_string()), false);
        assert_eq!(Solution::is_number("e".to_string()), false);
        assert_eq!(Solution::is_number("0".to_string()), true);
        assert_eq!(Solution::is_number("3.".to_string()), true);
        assert_eq!(Solution::is_number(".1".to_string()), true);
        assert_eq!(Solution::is_number("1E9".to_string()), true);
    }
}
