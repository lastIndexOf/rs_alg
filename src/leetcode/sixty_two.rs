pub struct Solution;

impl Solution {
    // 递归 超时
    // pub fn unique_paths(m: i32, n: i32) -> i32 {
    //     let mut map = std::collections::HashMap::new();
    //     return Self::do_unique_paths((m, n), &mut map);
    // }

    // fn do_unique_paths(
    //     (m, n): (i32, i32),
    //     map: &mut std::collections::HashMap<(i32, i32), i32>,
    // ) -> i32 {
    //     if let Some(res) = map.get(&(m, n)) {
    //         return *res;
    //     }

    //     if m == 1 || n == 1 {
    //         return 1;
    //     }

    //     let res = Self::unique_paths(m - 1, n) + Self::unique_paths(m, n - 1);

    //     map.insert((m, n), res);
    //     res
    // }

    // 动态规划
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut res = vec![vec![1_i32; n as usize]; m as usize];

        for y in (0..(m - 1)).rev() {
            for x in (0..(n - 1)).rev() {
                let y = y as usize;
                let x = x as usize;
                res[y][x] = res[y][x + 1] + res[y + 1][x];
            }
        }

        res[0][0]
    }
}

#[cfg(test)]
mod sixty_two_test {
    use super::*;

    #[test]
    fn test_sixty_two() {
        assert_eq!(Solution::unique_paths(3, 7), 28);
        assert_eq!(Solution::unique_paths(3, 2), 3);
        assert_eq!(Solution::unique_paths(7, 3), 28);
        assert_eq!(Solution::unique_paths(3, 3), 6);
        assert_eq!(Solution::unique_paths(23, 12), 193536720);
    }
}
