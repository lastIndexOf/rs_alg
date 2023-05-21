pub struct Solution;

impl Solution {
    // 滑动窗口
    // [1, 2, 3, 4]
    // [1, 2, 3]
    //    [2, 3, 4]
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        let mut res = vec![];

        let word_len = words[0].len();
        let word_count = words.len();
        let sub_str_len = word_count * word_len;

        if s.len() < sub_str_len {
            return vec![];
        }

        for i in 0..=(s.len() - sub_str_len) {
            map.clear();
            for key in &words {
                *map.entry(&key[..]).or_insert(0) += 1;
            }
            let sub_str = &s[i..(i + sub_str_len)];
            for j in 0..word_count {
                let start = j * word_len;
                if map.contains_key(&sub_str[start..(start + word_len)]) {
                    *(map.get_mut(&sub_str[start..(start + word_len)]).unwrap()) -= 1;
                } else {
                    break;
                }
            }

            if map.iter().all(|(_, count)| *count == 0) {
                res.push(i as i32);
            }
        }

        res
    }
}

#[cfg(test)]
mod thirty_test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_thirty() {
        assert_eq!(
            Solution::find_substring(
                "barfoothefoobarman".to_string(),
                vec!["foo".to_string(), "bar".to_string()]
            ),
            [0, 9]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "word".to_string()
                ]
            ),
            vec![]
        );
        assert_eq!(
            Solution::find_substring(
                "barfoofoobarthefoobarman".to_string(),
                vec!["bar".to_string(), "foo".to_string(), "the".to_string()]
            ),
            vec![6, 9, 12]
        );
        assert_eq!(
            Solution::find_substring(
                "wordgoodgoodgoodbestword".to_string(),
                vec![
                    "word".to_string(),
                    "good".to_string(),
                    "best".to_string(),
                    "good".to_string()
                ]
            ),
            vec![8]
        );
    }
}
