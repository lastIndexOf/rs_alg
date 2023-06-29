pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut start = 1;
        let mut end = x;

        let mut middle = (end + start) / 2;

        while start < end - 1 {
            if middle > x / middle {
                end = middle;
            } else {
                start = middle;
            }
            middle = (end + start) / 2;
        }

        middle
    }
}

#[cfg(test)]
mod sixty_nine_test {
    use super::*;

    #[test]
    fn test_sixty_nine() {
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
        assert_eq!(Solution::my_sqrt(1), 1);
        assert_eq!(Solution::my_sqrt(2147395599), 46339);
    }
}
