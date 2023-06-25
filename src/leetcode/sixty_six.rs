pub struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 1;

        for item in digits.iter_mut().rev() {
            let res = *item + carry;
            if res > 9 {
                *item = res - 10;
                carry = 1;
            } else {
                carry = 0;
                *item = res;
            }
        }

        if carry == 1 {
            digits.insert(0, 1);
        }

        digits
    }
}

#[cfg(test)]
mod sixty_six_test {
    use super::*;

    #[test]
    fn test_sixty_six() {
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), [4, 3, 2, 2]);
    }
}
