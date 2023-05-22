pub struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = vec![];
        let chars = s.chars();
        let mut max = 0;
        let mut start = 0;

        for (current, cr) in chars.enumerate() {
            match cr {
                '(' => {
                    stack.push(current);
                }
                ')' => {
                    if stack.pop().is_none() {
                        let len = current - start + 1;
                        max = std::cmp::max(max, len);
                    } else {
                        start = current + 1;
                    }
                }
                _ => {}
            }
        }

        max as i32
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
