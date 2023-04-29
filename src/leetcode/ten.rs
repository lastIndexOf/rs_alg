pub struct Solution;

const DOT: char = '.';
const STAR: char = '*';

impl Solution {
    pub fn is_match(s: &str, p: &str) -> bool {
        if p.is_empty() {
            return s.is_empty();
        }

        let first_match = !s.is_empty()
            && (p.chars().nth(0).unwrap() == s.chars().nth(0).unwrap()
                || p.chars().nth(0).unwrap() == DOT);

        if p.len() > 1 && p.chars().nth(1).unwrap() == STAR {
            Solution::is_match(&s[..], &p[2..])
                || (first_match && Solution::is_match(&s[1..], &p[..]))
        } else {
            first_match && Solution::is_match(&s[1..], &p[1..])
        }
    }
}

#[cfg(test)]
mod ten_test {
    use super::*;

    #[test]
    fn test_ten() {
        assert_eq!(Solution::is_match("aa", "a"), false);
        assert_eq!(Solution::is_match("aa", "a*"), true);
        assert_eq!(Solution::is_match("ab", ".*"), true);
    }
}
