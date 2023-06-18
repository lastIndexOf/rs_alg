pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.push(new_interval);
        Self::merge(intervals)
    }

    fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort_by_key(|item| item[0]);
        let mut res = Vec::with_capacity(intervals.len() * 2);
        let intervals = intervals.into_iter().flatten().collect::<Vec<_>>();
        let len = intervals.len();

        let mut start = 0;
        let mut end = 1;

        let mut end_item = intervals[end];

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
mod fifty_seven_test {
    #[test]
    fn test_fifty_seven() {}
}
