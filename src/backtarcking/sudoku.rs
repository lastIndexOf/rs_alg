/*
    A Rust implementation of Sudoku solver using Backtracking.
    GeeksForGeeks: https://www.geeksforgeeks.org/sudoku-backtracking-7/
*/

pub struct Sudoku {
    board: [[u8; 9]; 9],
}
impl Sudoku {
    pub fn new(board: [[u8; 9]; 9]) -> Self {
        Self { board }
    }

    pub fn find_empty_node(&self) -> Option<(usize, usize)> {
        for y in 0..9 {
            for x in 0..9 {
                if self.board[y][x] == 0 {
                    return Some((y, x));
                }
            }
        }

        None
    }

    pub fn solve(&mut self) -> bool {
        if let Some((y, x)) = self.find_empty_node() {
            // 找到了一个待填的点
            for val in 1..=9 {
                if self.check((y, x), val) {
                    self.board[y][x] = val;
                    if self.solve() {
                        return true;
                    }
                    self.board[y][x] = 0;
                }
            }
        } else {
            return true;
        }

        false
    }

    pub fn check(&self, (y, x): (usize, usize), val: u8) -> bool {
        // 横向
        for i in 0..9 {
            if self.board[i][x] == val {
                return false;
            }
        }

        // 纵向
        for i in 0..9 {
            if self.board[y][i] == val {
                return false;
            }
        }

        // 3x3子网格
        let y_start = y / 3 * 3;
        let x_start = x / 3 * 3;
        for y_axis in y_start..(y_start + 3) {
            for x_axis in x_start..(x_start + 3) {
                if self.board[y_axis][x_axis] == val {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku_correct() {
        let board: [[u8; 9]; 9] = [
            [3, 0, 6, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];

        let board_result = [
            [3, 1, 6, 5, 7, 8, 4, 9, 2],
            [5, 2, 9, 1, 3, 4, 7, 6, 8],
            [4, 8, 7, 6, 2, 9, 5, 3, 1],
            [2, 6, 3, 4, 1, 5, 9, 8, 7],
            [9, 7, 4, 8, 6, 3, 1, 2, 5],
            [8, 5, 1, 7, 9, 2, 6, 4, 3],
            [1, 3, 8, 9, 4, 7, 2, 5, 6],
            [6, 9, 2, 3, 5, 1, 8, 7, 4],
            [7, 4, 5, 2, 8, 6, 3, 1, 9],
        ];

        let mut sudoku = Sudoku::new(board);
        let is_solved = sudoku.solve();

        assert!(is_solved);
        assert_eq!(sudoku.board, board_result);
    }

    #[test]
    fn test_sudoku_incorrect() {
        let board: [[u8; 9]; 9] = [
            [6, 0, 3, 5, 0, 8, 4, 0, 0],
            [5, 2, 0, 0, 0, 0, 0, 0, 0],
            [0, 8, 7, 0, 0, 0, 0, 3, 1],
            [0, 0, 3, 0, 1, 0, 0, 8, 0],
            [9, 0, 0, 8, 6, 3, 0, 0, 5],
            [0, 5, 0, 0, 9, 0, 6, 0, 0],
            [1, 3, 0, 0, 0, 0, 2, 5, 0],
            [0, 0, 0, 0, 0, 0, 0, 7, 4],
            [0, 0, 5, 2, 0, 6, 3, 0, 0],
        ];

        let mut sudoku = Sudoku::new(board);
        let is_solved = sudoku.solve();

        assert!(!is_solved);
    }
}
