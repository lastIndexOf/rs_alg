pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for (x, x_value) in board.iter().enumerate() {
            for (y, value) in x_value.into_iter().enumerate() {
                if value != &'.' && !Self::valid_rule(&board, (x, y), *value) {
                    return false;
                }
            }
        }

        true
    }

    fn valid_rule(board: &Vec<Vec<char>>, (x, y): (usize, usize), value: char) -> bool {
        for i in 0..9 {
            if board[x][i] == value && i != y {
                return false;
            }
        }

        for j in 0..9 {
            if board[j][y] == value && j != x {
                return false;
            }
        }

        // nine grid

        let x_start = x / 3 * 3;
        let y_start = y / 3 * 3;

        for i in x_start..(x_start + 3) {
            for j in y_start..(y_start + 3) {
                if board[i][j] == value && i != x && y != j {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod thirty_six_test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_thirty_six() {
        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
            true
        );

        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
            ]),
            false,
        );

        assert_eq!(
            Solution::is_valid_sudoku(vec![
                vec!['.', '8', '7', '6', '5', '4', '3', '2', '1'],
                vec!['2', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['3', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['4', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['5', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['6', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['7', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['8', '.', '.', '.', '.', '.', '.', '.', '.'],
                vec!['9', '.', '.', '.', '.', '.', '.', '.', '.']
            ]),
            true,
        );
    }
}
