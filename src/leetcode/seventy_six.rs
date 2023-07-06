pub struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() {
            return String::from("");
        }

        let mut map =
            t.as_bytes()
                .into_iter()
                .fold(std::collections::HashMap::new(), |mut acc, next| {
                    *acc.entry(next).or_insert(0) += 1;
                    acc
                });

        let s = s.as_bytes();
        let mut min_left = 0;
        let mut min_right = s.len() - 1;
        let mut total_len = t.as_bytes().len();
        let mut has_result = false;

        let mut left = 0;
        for (right, cr) in s.iter().enumerate() {
            if map.contains_key(cr) {
                let current = map.entry(cr).or_default();

                if *current > 0 {
                    total_len -= 1;
                }

                *current -= 1;
            }

            if total_len == 0 {
                loop {
                    let has_key = map.contains_key(&s[left]);
                    if has_key && map[&s[left]] >= 0 {
                        break;
                    }
                    if has_key {
                        *map.entry(&s[left]).or_default() += 1;
                    }
                    left += 1;
                }

                if right - left <= min_right - min_left {
                    min_right = right;
                    min_left = left;
                    has_result = true;
                }

                *map.entry(&s[left]).or_default() += 1;
                left += 1;
                total_len += 1;
            }
        }

        if has_result {
            String::from_utf8_lossy(&s[min_left..=min_right]).to_string()
        } else {
            String::from("")
        }
    }
}

#[cfg(test)]
mod seventy_six_test {
    use super::*;

    #[test]
    fn test_seventy_six() {
        assert_eq!(
            Solution::min_window("ADOBECODEBANC".to_string(), "ABC".to_string()),
            "BANC"
        );
        assert_eq!(Solution::min_window("a".to_string(), "a".to_string()), "a");
        assert_eq!(Solution::min_window("a".to_string(), "aa".to_string()), "");
        assert_eq!(Solution::min_window("a".to_string(), "b".to_string()), "");
    }
}
