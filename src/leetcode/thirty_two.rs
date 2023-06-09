pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.as_bytes();
        let len = s.len();
        let mut res = 0;
        let mut start = 0;
        let mut stack = vec![];

        for idx in 0..len {
            if s[idx] == b'(' {
                stack.push(idx);
            } else {
                match stack.pop() {
                    Some(_) => {
                        if stack.is_empty() {
                            res = std::cmp::max(res, idx - start + 1);
                        } else {
                            res = std::cmp::max(res, idx - stack.last().unwrap());
                        }
                    }
                    None => {
                        start = idx + 1;
                    }
                }
            }
        }

        res as i32
    }
}

#[cfg(test)]
mod thirty_two_test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_thirty_two() {
        assert_eq!(Solution::longest_valid_parentheses("(()".to_string()), 2);
        assert_eq!(Solution::longest_valid_parentheses(")()())".to_string()), 4);
        assert_eq!(Solution::longest_valid_parentheses("".to_string()), 0);
        assert_eq!(Solution::longest_valid_parentheses("()(())".to_string()), 6);
        assert_eq!(Solution::longest_valid_parentheses("()(()".to_string()), 2);
        assert_eq!(
            Solution::longest_valid_parentheses(")(((((()())()()))()(()))(".to_string()),
            22
        );
        assert_eq!(Solution::longest_valid_parentheses("(()()".to_string()), 4);
    }
}
