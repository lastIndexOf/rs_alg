pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for cr in s.chars() {
            match cr {
                '(' | '[' | '{' => stack.push(cr),
                ')' => {
                    if let Some('(') = stack.pop() {
                    } else {
                        return false;
                    }
                }
                ']' => {
                    if let Some('[') = stack.pop() {
                    } else {
                        return false;
                    }
                }
                '}' => {
                    if let Some('{') = stack.pop() {
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod twenty_test {
    use super::*;

    #[test]
    fn test_twenty() {
        assert!(Solution::is_valid(String::from("()")));
        assert!(Solution::is_valid(String::from("()[]{}")));
        assert!(!Solution::is_valid(String::from("(]")));
        assert!(!Solution::is_valid(String::from("([)]")));
        assert!(Solution::is_valid(String::from("(([]){})")));
    }
}
