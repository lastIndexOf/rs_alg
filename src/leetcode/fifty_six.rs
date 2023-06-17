pub struct Solution;

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by_key(|item| item[0]);
        let mut res = vec![];
        let intervals = intervals.into_iter().flatten().collect::<Vec<_>>();
        let len = intervals.len();

        let mut start = 0;
        let mut end = 1;

        let mut end_item = intervals[end];

        // println!("{intervals:?}");
        loop {
            while end + 1 < len && end_item >= intervals[end + 1] {
                end_item = if end_item > intervals[end + 2] {
                    end_item
                } else {
                    intervals[end + 2]
                };

                end = end + 2;
            }

            res.push(intervals[start]);
            res.push(end_item);

            start = end + 1;
            end = start + 1;

            if start >= len {
                break;
            }

            end_item = intervals[end];
        }

        res.chunks(2).map(|v| v.to_vec()).collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod fifty_six_test {
    use super::*;

    #[test]
    fn test_fifty_six() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            [[1, 6], [8, 10], [15, 18]]
        );
        assert_eq!(Solution::merge(vec![vec![1, 4], vec![4, 5]]), [[1, 5]]);
        assert_eq!(Solution::merge(vec![vec![1, 4], vec![0, 4]]), [[0, 4]]);
        assert_eq!(Solution::merge(vec![vec![1, 4], vec![2, 3]]), [[1, 4]]);
        assert_eq!(
            Solution::merge(vec![
                vec![2, 3],
                vec![4, 5],
                vec![6, 7],
                vec![8, 9],
                vec![1, 10]
            ]),
            [[1, 10]]
        );
    }
}
