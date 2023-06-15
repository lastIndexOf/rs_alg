pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut top = 0_i32;
        let mut left = 0_i32;
        let mut right = (matrix[0].len() - 1) as i32;
        let mut bottom = (matrix.len() - 1) as i32;

        let capacity = right * bottom;
        let mut res = Vec::with_capacity(capacity as usize);

        loop {
            // println!("{top} {bottom} {left} {right} {res:?}");
            for x in left..=right {
                res.push(matrix[top as usize][x as usize]);
            }

            if top <= bottom {
                top += 1;
            }

            for y in top..=bottom {
                res.push(matrix[y as usize][right as usize]);
            }

            if left <= right {
                right -= 1;
            }

            if top <= bottom {
                for x in (left..=right).rev() {
                    res.push(matrix[bottom as usize][x as usize]);
                }

                bottom -= 1;
            }

            if left <= right {
                for y in (top..=bottom).rev() {
                    res.push(matrix[y as usize][left as usize]);
                }

                left += 1;
            }

            if left > right || top > bottom {
                break;
            }
        }

        res
    }
}

#[cfg(test)]
mod fifty_four_test {
    use super::*;

    #[test]
    fn test_fifty_four() {
        assert_eq!(
            Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
            [1, 2, 3, 6, 9, 8, 7, 4, 5]
        );
        assert_eq!(
            Solution::spiral_order(vec![
                vec![1, 2, 3, 4],
                vec![5, 6, 7, 8],
                vec![9, 10, 11, 12]
            ]),
            [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7]
        );
        assert_eq!(Solution::spiral_order(vec![vec![1]]), [1]);
        assert_eq!(Solution::spiral_order(vec![vec![3], vec![2]]), [3, 2]);
        assert_eq!(Solution::spiral_order(vec![vec![6, 9, 7]]), [6, 9, 7]);
    }
}
