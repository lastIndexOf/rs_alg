pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(n as usize);

        for _ in 0..n {
            res.push(vec![Default::default(); n as usize]);
        }

        let [mut top, mut left, mut bottom, mut right] = [0, 0, n as usize - 1, n as usize - 1];

        let mut current = 1;
        let max = n * n;
        loop {
            // if current <= max {}
            for x in left..=right {
                res[top][x] = current;
                current += 1;
            }

            for y in (top + 1)..=bottom {
                res[y][right] = current;
                current += 1;
            }

            for x in (left..right).rev() {
                res[bottom][x] = current;
                current += 1;
            }

            for y in ((top + 1)..bottom).rev() {
                res[y][left] = current;
                current += 1;
            }

            if current > max {
                break;
            }

            top += 1;
            bottom -= 1;
            left += 1;
            right -= 1;
        }

        res
    }
}

#[cfg(test)]
mod fifty_nine_test {
    use super::*;

    #[test]
    fn test_fifty_nine() {
        assert_eq!(
            Solution::generate_matrix(3),
            [[1, 2, 3], [8, 9, 4], [7, 6, 5]]
        );
    }
}
