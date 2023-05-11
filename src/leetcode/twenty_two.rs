pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = vec![];
        // 回溯法，可以解决
        // 还有一种思路，把结果看成二叉树，使用 DFS
        do_generate_parenthesis(&mut res, &mut vec![], &mut String::new(), 0, 2 * n);
        res
    }
}

fn do_generate_parenthesis(
    res: &mut Vec<String>,
    stack: &mut Vec<char>,
    current_str: &mut String,
    start: i32,
    n: i32,
) {
    if start == n {
        if stack.is_empty() {
            res.push(current_str.clone());
        }
        return;
    }

    stack.push('(');
    current_str.push('(');
    do_generate_parenthesis(res, stack, current_str, start + 1, n);
    current_str.pop();
    stack.pop();

    if !stack.is_empty() && stack.last().unwrap() == &'(' {
        current_str.push(')');
        stack.pop();
        do_generate_parenthesis(res, stack, current_str, start + 1, n);
        stack.push('(');
        current_str.pop();
    }
}

#[cfg(test)]
mod twenty_two_test {
    use super::*;

    #[test]
    fn test_twenty_two() {
        assert_eq!(Solution::generate_parenthesis(1), vec![String::from("()"),]);
        assert_eq!(
            Solution::generate_parenthesis(3),
            vec![
                String::from("((()))"),
                String::from("(()())"),
                String::from("(())()"),
                String::from("()(())"),
                String::from("()()()")
            ]
        );
    }
}
