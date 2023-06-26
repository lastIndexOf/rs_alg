pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::with_capacity(std::cmp::max(a.len(), b.len()));
        let mut a = a.as_bytes().iter().rev();
        let mut b = b.as_bytes().iter().rev();

        let mut carry = 0;
        loop {
            match (a.next(), b.next()) {
                (Some(a), Some(b)) => {
                    let a = *a - b'0';
                    let b = *b - b'0';

                    let mut res = a + b + carry;

                    if res > 1 {
                        carry = 1;
                        res -= 2;
                    } else {
                        carry = 0;
                    }

                    result.insert(0, (res + b'0') as char);
                    println!("{a}, {b}, {res}");
                }
                (Some(a), None) => {
                    let a = *a - b'0';
                    let mut res = a + carry;

                    if res > 1 {
                        carry = 1;
                        res -= 2;
                    } else {
                        carry = 0;
                    }

                    result.insert(0, (res + b'0') as char);
                }
                (None, Some(b)) => {
                    let b = *b - b'0';
                    let mut res = b + carry;

                    if res > 1 {
                        carry = 1;
                        res -= 2;
                    } else {
                        carry = 0;
                    }

                    result.insert(0, (res + b'0') as char);
                }
                _ => {
                    return if carry == 1 {
                        result.insert(0, b'1' as char);
                        result
                    } else {
                        result
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod sixty_seven_test {
    use super::*;

    #[test]
    fn test_sixty_seven() {
        assert_eq!(
            Solution::add_binary("11".to_string(), "1".to_string()),
            "100"
        );
    }
}
