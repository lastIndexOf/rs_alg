pub struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let num1 = num1.as_bytes();
        let num2 = num2.as_bytes();
        let len = num2.len() + num1.len();

        let mut res = vec![0; len];
        for (i, num) in num2.iter().rev().enumerate() {
            let num = num - b'0';
            let mut big_part = 0;

            for (idx, parent) in num1.iter().rev().enumerate() {
                let parent = parent - b'0';
                let tmp = parent * num + big_part;

                let small_part = tmp % 10;
                big_part = tmp / 10;

                res[len - 1 - idx - i] += small_part;

                if res[len - 1 - idx - i] >= 10 {
                    big_part += res[len - 1 - idx - i] / 10;
                    res[len - 1 - idx - i] = res[len - 1 - idx - i] % 10;
                }
            }

            res[len - 1 - num1.len() - i] = big_part;

            // println!("res = {res:?}");
        }

        let res = res
            .into_iter()
            .skip_while(|cr| *cr == 0)
            .map(|cr| (cr + b'0') as char)
            .collect::<String>();

        if res.is_empty() {
            return "0".to_string();
        }

        res
    }
}

#[cfg(test)]
mod forty_three_test {
    use super::*;

    #[test]
    fn test_forty_three() {
        assert_eq!(
            Solution::multiply("2".to_string(), "3".to_string()),
            "6".to_string()
        );
        assert_eq!(
            Solution::multiply("123".to_string(), "456".to_string()),
            "56088".to_string()
        );
        assert_eq!(
            Solution::multiply("123456789".to_string(), "987654321".to_string()),
            "121932631112635269".to_string()
        );
        assert_eq!(
            Solution::multiply("2".to_string(), "3".to_string()),
            "6".to_string()
        );
    }
}
