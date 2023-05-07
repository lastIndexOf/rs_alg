pub struct Solution;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let map = init_map();

        let nums = digits
            .as_bytes()
            .iter()
            .map(|item| *map.get(&(*item - b'0')).unwrap())
            .collect::<Vec<_>>();

        let mut res = Vec::<String>::new();
        any_combination(&nums, &mut res, &mut String::new(), 0);

        res
    }
}

fn init_map<'a>() -> std::collections::HashMap<u8, &'a [&'static str]> {
    let mut map: std::collections::HashMap<u8, &[&str]> = std::collections::HashMap::new();

    map.entry(2).or_insert(&["a", "b", "c"]);
    map.entry(3).or_insert(&["d", "e", "f"]);
    map.entry(4).or_insert(&["g", "h", "i"]);
    map.entry(5).or_insert(&["j", "k", "l"]);
    map.entry(6).or_insert(&["m", "n", "o"]);
    map.entry(7).or_insert(&["p", "q", "r", "s"]);
    map.entry(8).or_insert(&["t", "u", "v"]);
    map.entry(9).or_insert(&["w", "x", "y", "z"]);

    map
}

fn any_combination(
    nums: &Vec<&[&str]>,
    res: &mut Vec<String>,
    current_str: &mut String,
    start: usize,
) {
    if start == nums.len() {
        if !current_str.is_empty() {
            res.push(current_str.clone());
        }
        return;
    }

    for &num in nums[start].iter() {
        current_str.push_str(num);
        any_combination(nums, res, current_str, start + 1);
        current_str.pop();
    }
}

#[cfg(test)]
mod seventeen_test {
    use super::*;

    #[test]
    fn test_seventeen() {
        assert_eq!(
            Solution::letter_combinations("23".to_string()),
            ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
        );
        assert_eq!(
            Solution::letter_combinations("2".to_string()),
            ["a", "b", "c"]
        );
        assert_eq!(
            Solution::letter_combinations("".to_string()),
            Vec::<String>::new()
        );
    }
}
