pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut map = std::collections::HashMap::new();
        Self::is_match_str(&s.as_bytes(), &p.as_bytes(), &mut map)
    }

    fn is_match_str<'a>(
        s: &'a [u8],
        p: &'a [u8],
        map: &mut std::collections::HashMap<&'a [u8], bool>,
    ) -> bool {
        if let Some(_) = map.get(&s) {
            return false;
        }

        let mut si = 0;
        let mut pi = 0;

        while si < s.len() && pi < p.len() {
            if s[si] == p[pi] || p[pi] == b'?' {
                si += 1;
                pi += 1;
                continue;
            }
            if p[pi] == b'*' {
                return Self::is_match_str(&s[si..], &p[(pi + 1)..], map)
                    || Self::is_match_str(&s[(si + 1)..], &p[pi..], map);
            }

            map.insert(s, false);
            return false;
        }

        if si >= s.len() {
            if pi >= p.len() {
                return true;
            }

            for i in pi..p.len() {
                if p[i] != b'*' {
                    map.insert(s, false);
                    return false;
                }
            }

            return true;
        }

        if pi >= p.len() {
            map.insert(s, false);
            return false;
        }

        false
    }
}

#[cfg(test)]
mod forty_four_test {
    use super::*;

    #[test]
    fn test_forty_four() {
        assert_eq!(Solution::is_match("aa".to_string(), "a".to_string()), false);
        assert_eq!(Solution::is_match("aa".to_string(), "*".to_string()), true);
        assert_eq!(
            Solution::is_match("cb".to_string(), "?a".to_string()),
            false
        );
        assert_eq!(
            Solution::is_match("adceb".to_string(), "*a*b".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("".to_string(), "******".to_string()),
            true
        );
        assert_eq!(Solution::is_match("a".to_string(), "aa".to_string()), false);
        assert_eq!(
            Solution::is_match("abcabczzzde".to_string(), "*abc???de*".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match("abefcdgiescdfimde".to_string(), "ab*cd?i*de".to_string()),
            true
        );
        assert_eq!(
            Solution::is_match(
                "bbbbbbbabbaabbabbbbaaabbabbabaaabbababbbabbbabaaabaab".to_string(),
                "b*b*ab**ba*b**b***bba".to_string()
            ),
            false
        );
    }
}
