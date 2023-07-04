pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();

        let mut y = 0;
        loop {
            if matrix[y][0] > target {
                if y > 0 {
                    y -= 1;
                    break;
                }

                return false;
            }

            if matrix[y][0] == target {
                return true;
            }

            if y + 1 >= m {
                break;
            }

            y += 1;
        }

        if let Some(_) = matrix[y].iter().find(|item| **item == target) {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod seventy_four_test {
    use super::*;

    #[test]
    fn test_seventy_four() {
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                3
            ),
            true
        );
        assert_eq!(
            Solution::search_matrix(
                vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
                13
            ),
            false
        );
    }
}
