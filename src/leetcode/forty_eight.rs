pub struct Solution;

impl Solution {
    // 蛮力
    // pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    //     // (0,0) (3,0) (3,3) (0,3)
    //     // (0,1) (2,0) (3,2) (1,3)
    //     // (0,2) (1,0) (3,1) (2,3)
    //     // (0,3) (0,0) (3,0) (3,3)
    //     let y_len = matrix.len();
    //     let x_len = matrix[0].len();

    //     for y in 0..y_len {
    //         let origin = (y, y);
    //         for x in y..(x_len - y - 1) {
    //             let matrix_len = x_len - y - y;

    //             // println!("(x,y) = ({x}, {y}) matrix_len = {matrix_len}, origin = {origin:?}");

    //             if matrix_len <= 1 {
    //                 return;
    //             }

    //             unsafe {
    //                 std::ptr::swap(
    //                     &mut matrix[y][x] as *mut i32,
    //                     &mut matrix[y + matrix_len - 1 - (x - origin.1)][origin.1] as *mut i32,
    //                 );
    //                 std::ptr::swap(
    //                     &mut matrix[y + matrix_len - 1 - (x - origin.1)][origin.1] as *mut i32,
    //                     &mut matrix[y + matrix_len - 1][origin.1 + matrix_len - 1 - (x - origin.1)]
    //                         as *mut i32,
    //                 );
    //                 std::ptr::swap(
    //                     &mut matrix[y + matrix_len - 1][origin.1 + matrix_len - 1 - (x - origin.1)]
    //                         as *mut i32,
    //                     &mut matrix[y + (x - origin.1)][origin.1 + matrix_len - 1] as *mut i32,
    //                 );
    //             }
    //         }
    //     }
    // }

    // 先转置，再镜像
    // 180度可以先上下镜像，再左右镜像
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        for y in 0..matrix.len() {
            for x in y..matrix[y].len() {
                unsafe {
                    std::ptr::swap(&mut matrix[y][x] as *mut i32, &mut matrix[x][y] as *mut i32)
                };
            }
        }

        for y in 0..matrix.len() {
            let len = matrix[y].len();
            for x in 0..(len / 2) {
                unsafe {
                    std::ptr::swap(
                        &mut matrix[y][x] as *mut i32,
                        &mut matrix[y][len - 1 - x] as *mut i32,
                    )
                };
            }
        }
    }
}

#[cfg(test)]
mod forty_eight_test {
    use super::*;

    #[test]
    fn test_forty_eight() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        assert_eq!(matrix, [[7, 4, 1], [8, 5, 2], [9, 6, 3]]);
        // [
        //     [1,9,7],
        //     [4,5,6],
        //     [2,8,3],
        // ]
        let mut matrix = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        assert_eq!(
            matrix,
            [
                [15, 13, 2, 5],
                [14, 3, 4, 1],
                [12, 6, 8, 9],
                [16, 7, 10, 11]
            ]
        );
    }
}
