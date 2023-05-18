pub struct Solution;

impl Solution {
    // pub fn str_str(haystack: String, needle: String) -> i32 {
    //     match haystack.find(needle.as_str()) {
    //         Some(idx) => idx as i32,
    //         None => -1,
    //     }
    // }
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.chars().collect::<Vec<_>>();
        let needle = needle.chars().collect::<Vec<_>>();

        if haystack.len() < needle.len() {
            return -1;
        }

        let mut haystack_idx = 0;
        let mut needle_idx = 0;
        let mut res = -1;
        let len = haystack.len();

        while haystack_idx < len {
            if needle[needle_idx] == haystack[haystack_idx] {
                if needle_idx == 0 {
                    res = haystack_idx as i32;
                }
                needle_idx += 1;
                if needle_idx == needle.len() {
                    break;
                }
            } else {
                if res != -1 {
                    haystack_idx = res as usize;
                    res = -1;
                }
                needle_idx = 0;
            }
            haystack_idx += 1;
        }

        if needle_idx == needle.len() {
            res
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod twenty_eight_test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_twenty_eight() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
        assert_eq!(
            Solution::str_str("mississippi".to_string(), "issip".to_string()),
            4
        );
    }
}
