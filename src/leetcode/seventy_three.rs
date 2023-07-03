pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] == 0 {
                    Self::clear_row_and_col(matrix, (y, x));
                }
            }
        }

        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                matrix[y][x] = if matrix[y][x] == -999999 {
                    0
                } else {
                    matrix[y][x]
                }
            }
        }
    }

    fn clear_row_and_col(matrix: &mut Vec<Vec<i32>>, (y, x): (usize, usize)) {
        for i in 0..matrix.len() {
            if i != y && matrix[i][x] != 0 {
                matrix[i][x] = -999999;
            }
        }

        for j in 0..matrix[y].len() {
            if j != x && matrix[y][j] != 0 {
                matrix[y][j] = -999999;
            }
        }
    }
}

#[cfg(test)]
mod seventy_three_test {
    use super::*;

    #[test]
    fn test_seventy_three() {
        let mut v = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
        Solution::set_zeroes(&mut v);
        assert_eq!(v, [[1, 0, 1], [0, 0, 0], [1, 0, 1]]);

        let mut v = vec![vec![0, 1, 2, 0], vec![3, 4, 5, 2], vec![1, 3, 1, 5]];
        Solution::set_zeroes(&mut v);
        assert_eq!(v, [[0, 0, 0, 0], [0, 4, 5, 0], [0, 3, 1, 0]]);

        let mut v = vec![vec![-1], vec![2], vec![3]];
        Solution::set_zeroes(&mut v);
        assert_eq!(v, [[-1], [2], [3]]);
    }
}
