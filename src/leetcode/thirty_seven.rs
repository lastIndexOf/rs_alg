pub struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
        match Self::find_empty(board) {
            Some((x, y)) => {
                for i in 1..=9 {
                    let value = std::char::from_digit(i, 10).unwrap();
                    board[x][y] = value;

                    if Self::valid_rule(board, (x, y), value) && Self::solve_sudoku(board) {
                        return true;
                    }

                    board[x][y] = '.';
                }

                false
            }
            None => true,
        }
    }

    fn find_empty(board: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        for (x, val) in board.into_iter().enumerate() {
            for (y, val) in val.into_iter().enumerate() {
                if val == &'.' {
                    return Some((x, y));
                }
            }
        }

        None
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
mod thirty_seven_test {
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_thirty_seven() {
        assert_eq!(
            Solution::solve_sudoku(&mut vec![
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
            Solution::solve_sudoku(&mut vec![
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
    }
}
