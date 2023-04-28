/*
    In this problem, we want to determine all possible combinations of k
    numbers out of 1 ... n. We use backtracking to solve this problem.
    Time complexity: O(C(n,k)) which is O(n choose k) = O((n!/(k! * (n - k)!)))

    generate_all_combinations(n=4, k=2) => [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]]
*/

pub fn generate_all_combinations(n: i32, k: i32) -> Vec<Vec<i32>> {
    assert!(n >= k);

    let mut result = vec![];
    create_all_state(&mut result, &mut vec![], n, k, 1);
    result
}

fn create_all_state(
    result: &mut Vec<Vec<i32>>,
    current_list: &mut Vec<i32>,
    total: i32,
    level: i32,
    start: i32,
) {
    if level == 0 {
        result.push(current_list.clone());
        return;
    }

    for current in start..=(total - level + 1) {
        current_list.push(current);
        create_all_state(result, current_list, total, level - 1, current + 1);
        current_list.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let expected_res = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![2, 3],
            vec![2, 4],
            vec![3, 4],
        ];

        let res = generate_all_combinations(4, 2);

        println!("{:?}", generate_all_combinations(5, 3));

        assert_eq!(expected_res, res);
    }

    #[test]
    fn test_empty() {
        let expected_res: Vec<Vec<i32>> = vec![vec![]];

        let res = generate_all_combinations(0, 0);

        assert_eq!(expected_res, res);
    }
}
