pub struct Solution;

impl Solution {
    // pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
    //     let m = board.len();
    //     let n = board[0].len();

    //     let first = word.chars().next().unwrap();

    //     for y in 0..m {
    //         for x in 0..n {
    //             if board[y][x] == first {
    //                 if Self::dfs(&board, &word, (y, x)) {
    //                     return true;
    //                 }
    //             }
    //         }
    //     }

    //     false
    // }

    // fn dfs(board: &Vec<Vec<char>>, word: &str, start: (usize, usize)) -> bool {
    //     let m = board.len();
    //     let n = board[0].len();
    //     let word = word.chars().collect::<Vec<_>>();

    //     let mut visited: Vec<Vec<bool>> = vec![vec![true; n]; m];
    //     let mut paths = std::collections::VecDeque::with_capacity(m * n);
    //     let (y, x) = start;
    //     let mut next = 0;
    //     paths.push_back((y, x));

    //     let mut current_path: Vec<(usize, usize)> = vec![];

    //     while let Some((y, x)) = paths.pop_front() {
    //         if board[y][x] != word[next] {
    //             // if let Some((y, x)) = current_path.pop() {
    //             //     visited[y][x] = true;
    //             // }
    //             // if next > 0 {
    //             //     next -= 1;
    //             // }
    //             continue;
    //         }

    //         visited[y][x] = false;
    //         current_path.push((y, x));

    //         if let Some(_) = word.get(next + 1) {
    //             next = next + 1;

    //             if y > 0 && visited[y - 1][x] {
    //                 paths.push_front((y - 1, x));
    //             }

    //             if x > 0 && visited[y][x - 1] {
    //                 paths.push_front((y, x - 1));
    //             }

    //             if y + 1 < m && visited[y + 1][x] {
    //                 paths.push_front((y + 1, x));
    //             }

    //             if x + 1 < n && visited[y][x + 1] {
    //                 paths.push_front((y, x + 1));
    //             }
    //         } else {
    //             return true;
    //         }
    //     }

    //     false
    // }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let m = board.len();
        let n = board[0].len();

        let mut visited: Vec<Vec<bool>> = vec![vec![true; n]; m];
        let first = word.chars().next().unwrap();

        for y in 0..m {
            for x in 0..n {
                if board[y][x] == first {
                    if Self::back_trace(
                        &board,
                        &word.chars().collect::<Vec<char>>(),
                        &mut visited,
                        (y, x),
                    ) {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn back_trace(
        board: &Vec<Vec<char>>,
        word: &[char],
        visited: &mut Vec<Vec<bool>>,
        start: (usize, usize),
    ) -> bool {
        if word.is_empty() {
            return true;
        }

        let m = board.len();
        let n = board[0].len();
        let (y, x) = start;

        let cr = word[0];

        if cr != board[y][x] {
            return false;
        }

        if word.len() == 1 {
            return true;
        }

        visited[y][x] = false;

        if y > 0 && visited[y - 1][x] && Self::back_trace(board, &word[1..], visited, (y - 1, x)) {
            return true;
        }

        if x > 0 && visited[y][x - 1] && Self::back_trace(board, &word[1..], visited, (y, x - 1)) {
            return true;
        }

        if y + 1 < m
            && visited[y + 1][x]
            && Self::back_trace(board, &word[1..], visited, (y + 1, x))
        {
            return true;
        }

        if x + 1 < n
            && visited[y][x + 1]
            && Self::back_trace(board, &word[1..], visited, (y, x + 1))
        {
            return true;
        }

        visited[y][x] = true;

        false
    }
}

#[cfg(test)]
mod seventy_nine_test {
    use super::*;

    #[test]
    fn test_seventy_nine() {
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                String::from("ABCCED")
            ),
            true
        );
        assert_eq!(
            Solution::exist(
                vec![
                    vec!['A', 'B', 'C', 'E'],
                    vec!['S', 'F', 'C', 'S'],
                    vec!['A', 'D', 'E', 'E']
                ],
                String::from("ABCB")
            ),
            false
        );
        assert_eq!(
            Solution::exist(vec![vec!['a', 'b'], vec!['c', 'd']], String::from("abcd")),
            false
        );
        assert_eq!(
            Solution::exist(vec![vec!['a', 'a']], String::from("aaa")),
            false
        );
    }
}
