pub struct Solution;

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res = vec![];

        Self::do_solve_n_queens(&mut res, &mut vec![], 0, n);

        res
    }

    fn do_solve_n_queens(
        res: &mut Vec<Vec<String>>,
        current: &mut Vec<Vec<char>>,
        start: i32,
        n: i32,
    ) -> bool {
        if start == n {
            res.push(
                current
                    .clone()
                    .into_iter()
                    .map(|item| item.into_iter().collect::<String>())
                    .collect::<Vec<_>>(),
            );
            return true;
        }

        for y in start..n {
            for x in 0..n {
                let x = x as usize;
                let mut s = std::iter::repeat('.').take(n as usize).collect::<Vec<_>>();
                s[x] = 'Q';
                current.push(s);
                if Self::validate_rules(current, (y, x as i32), n)
                    && Self::do_solve_n_queens(res, current, y + 1, n)
                {}
                current.pop();
            }
        }

        false
    }

    fn validate_rules(current: &Vec<Vec<char>>, (y, x): (i32, i32), n: i32) -> bool {
        for i in 0..=y {
            if i != y && current[i as usize][x as usize] == 'Q' {
                return false;
            }

            for j in 0..n {
                if i - j == y - x && i != y && current[i as usize][j as usize] == 'Q' {
                    return false;
                }

                if i + j == y + x && i != y && current[i as usize][j as usize] == 'Q' {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod fifty_one_test {
    use super::*;

    #[test]
    fn test_fifty_one() {
        assert_eq!(
            Solution::solve_n_queens(4),
            [
                [".Q..", "...Q", "Q...", "..Q."],
                ["..Q.", "Q...", "...Q", ".Q.."]
            ]
        );
        assert_eq!(Solution::solve_n_queens(1), [["Q"]]);
    }
}
