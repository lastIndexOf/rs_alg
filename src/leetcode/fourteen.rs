pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut chars = strs.iter().map(|s| s.as_str().chars()).collect::<Vec<_>>();

        let mut res = String::from("");
        let mut common: char;

        loop {
            let next = chars[0].next();

            if next.is_none() {
                return res;
            } else {
                common = next.unwrap();
            }
            for idx in 1..chars.len() {
                let c = &mut chars[idx];
                match c.next() {
                    Some(next_char) if next_char == common => {}
                    _ => return res,
                }
            }

            res.push_str(&format!("{common}"));
        }
    }
}

#[cfg(test)]
mod fourteen_test {
    use super::*;

    #[test]
    fn test_fourteen() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
    }
}
