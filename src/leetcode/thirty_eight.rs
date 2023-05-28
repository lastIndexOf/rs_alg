pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return 1.to_string();
        }

        let prev = Self::count_and_say(n - 1);
        let mut res = String::with_capacity(prev.len());
        let mut prev = prev.chars();

        let mut start = prev.next().unwrap();
        let mut count = 1_u8;

        for i in prev {
            if start == i {
                count += 1;
            } else {
                res.push_str(format!("{count}{start}").as_str());
                start = i;
                count = 1;
            }
        }

        res.push_str(format!("{count}{start}").as_str());
        res.shrink_to_fit();
        res
    }
}

#[cfg(test)]
mod thirty_eight_test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_thirty_eight() {
        assert_eq!(Solution::count_and_say(1), "1");
        assert_eq!(Solution::count_and_say(4), "1211");
        assert_eq!(Solution::count_and_say(5), "111221");
    }
}
