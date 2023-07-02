pub struct Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let word1 = word1.as_bytes();
        let word2 = word2.as_bytes();
        let mut res = vec![vec![0; word2.len() + 1]; word1.len() + 1];

        for i in 0..=word1.len() {
            res[i][0] = i;
        }

        for i in 0..=word2.len() {
            res[0][i] = i;
        }

        for i in 1..=word1.len() {
            for j in 1..=word2.len() {
                if word1[i - 1] == word2[j - 1] {
                    res[i][j] = res[i - 1][j - 1];
                } else {
                    res[i][j] = std::cmp::min(
                        std::cmp::min(res[i - 1][j], res[i][j - 1]),
                        res[i - 1][j - 1],
                    ) + 1;
                }
            }
        }

        res[word1.len()][word2.len()] as i32
    }
}

#[cfg(test)]
mod seventy_two_test {
    use super::*;

    #[test]
    fn test_seventy_two() {
        assert_eq!(
            Solution::min_distance("horse".to_string(), "ros".to_string()),
            3
        );
        assert_eq!(
            Solution::min_distance("intention".to_string(), "execution".to_string()),
            5
        );
    }
}
