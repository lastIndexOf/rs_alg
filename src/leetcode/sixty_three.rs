pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();

        if obstacle_grid[m - 1][n - 1] == 1 || obstacle_grid[0][0] == 1 {
            return 0;
        }

        let mut res = vec![vec![0; n as usize]; m as usize];

        res[m - 1][n - 1] = 1;
        for y in (0..m).rev() {
            for x in (0..n).rev() {
                if obstacle_grid[y][x] == 1 {
                    res[y][x] = 0;
                } else {
                    if y == m - 1 {
                        if x < n - 1 {
                            res[y][x] = res[y][x + 1];
                        }
                    } else if x == n - 1 {
                        if y < m - 1 {
                            res[y][x] = res[y + 1][x];
                        }
                    } else {
                        match (obstacle_grid[y][x + 1], obstacle_grid[y + 1][x]) {
                            (1, 1) => res[y][x] = 0,
                            (1, 0) => res[y][x] = res[y + 1][x],
                            (0, 1) => res[y][x] = res[y][x + 1],
                            _ => res[y][x] = res[y][x + 1] + res[y + 1][x],
                        }
                    }
                }
            }
        }

        // println!("{res:?}");

        res[0][0]
    }
}

#[cfg(test)]
mod sixty_three_test {
    use super::*;

    #[test]
    fn test_sixty_three() {
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ]),
            2
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
            1
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![0, 1]]),
            0
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
            0
        );
        assert_eq!(
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 0, 0, 0, 1],
                vec![0, 0, 0, 1, 0],
                vec![0, 0, 0, 0, 0]
            ]),
            10
        );
    }
}
