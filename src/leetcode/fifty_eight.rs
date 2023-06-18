pub struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let split = s.split_whitespace().collect::<Vec<_>>();
        let res = split.last().unwrap();

        res.len() as i32
    }
}

#[cfg(test)]
mod fifty_eight_test {
    use super::*;

    #[test]
    fn test_fifty_eight() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
    }
}
