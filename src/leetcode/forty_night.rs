pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.is_empty() {
            return vec![];
        }

        if strs.len() == 1 {
            return vec![strs.clone()];
        }

        let mut map = std::collections::HashMap::new();

        for s in &strs {
            let mut s_cloned = s.as_bytes().to_vec().clone();
            s_cloned.sort();
            map.entry(s_cloned).or_insert(vec![]).push(s.clone());
        }

        map.into_values().collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod forty_nine_test {
    // use super::*;

    #[test]
    fn test_forty_nine() {
        // assert_eq!(
        //     Solution::group_anagrams(vec![
        //         "eat".to_string(),
        //         "tea".to_string(),
        //         "tan".to_string(),
        //         "ate".to_string(),
        //         "nat".to_string(),
        //         "bat".to_string()
        //     ]),
        //     vec![
        //         vec!["bat".to_string()],
        //         vec!["nat".to_string(), "tan".to_string()],
        //         vec!["ate".to_string(), "eat".to_string(), "tea".to_string()]
        //     ]
        // )
    }
}
