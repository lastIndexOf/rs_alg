pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let s = s.chars().collect::<Vec<_>>();
        let len = s.len();
        let mut res = 0;
        let mut start = 0;
        let mut stack = vec![];

        let mut idx = 0;

        while idx < len {
            if s[idx] == '(' {
                stack.push(idx);
                idx += 1;
            } else {
                match stack.pop() {
                    Some(end_idx) => {
                        if stack.is_empty() {
                            res = std::cmp::max(res, idx - start + 1);
                        } else {
                            res = std::cmp::max(res, idx - end_idx + 1);
                        }
                        idx += 1;
                    }
                    None => {
                        res = std::cmp::max(res, idx - start);
                        idx += 1;
                        start = idx;
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
    }
}
