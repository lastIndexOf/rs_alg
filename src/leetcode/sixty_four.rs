pub struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        let mut res = vec![vec![0; n]; m];

        for y in (0..m).rev() {
            for x in (0..n).rev() {
                if y + 1 < m && x + 1 < n {
                    res[y][x] = grid[y][x] + std::cmp::min(res[y + 1][x], res[y][x + 1]);
                } else if y + 1 < m {
                    res[y][x] = grid[y][x] + res[y + 1][x];
                } else if x + 1 < n {
                    res[y][x] = grid[y][x] + res[y][x + 1];
                } else {
                    res[y][x] = grid[y][x];
                }
            }
        }

        res[0][0]
    }
}

#[cfg(test)]
mod sixty_four_test {
    use super::*;

    #[test]
    fn test_sixty_four() {
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(vec![vec![1, 2, 3], vec![4, 5, 6]]),
            12
        );
    }
}
