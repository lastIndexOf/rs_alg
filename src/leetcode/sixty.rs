pub struct Solution;

impl Solution {
    // pub fn get_permutation(n: i32, k: i32) -> String {
    //     let count = (1..=n).product::<i32>();
    //     let mut nums = (1..=n).collect::<Vec<_>>();
    //     let mut res = Vec::with_capacity(count as usize);
    //     Self::get_all_permutation(&mut nums, &mut res, &mut Vec::with_capacity(n as usize));

    //     res.sort();
    //     res[k as usize - 1]
    //         .iter()
    //         .map(|item| std::char::from_digit(*item as u32, 10).unwrap())
    //         .collect::<String>()
    // }

    // fn get_all_permutation(nums: &mut Vec<i32>, res: &mut Vec<Vec<i32>>, current: &mut Vec<i32>) {
    //     if nums.is_empty() {
    //         res.push(current.clone());
    //         return;
    //     }

    //     for (idx, num) in nums.iter().enumerate() {
    //         current.push(*num);
    //         let mut nums = nums.clone();
    //         nums.swap_remove(idx);
    //         Self::get_all_permutation(&mut nums, res, current);
    //         current.pop();
    //     }
    // }
    pub fn get_permutation(n: i32, k: i32) -> String {
        let mut per_n_div_one_max_val = (1..=(n - 1)).product::<i32>();
        let mut bit_vec = Vec::with_capacity(n as usize);

        let mut idx = 1;
        let mut bit = 1;
        let mut k = k;

        while bit <= n {
            if idx * per_n_div_one_max_val < k {
                idx += 1;
                continue;
            }

            bit_vec.push(idx);
            k = k - (idx - 1) * per_n_div_one_max_val;
            idx = 1;
            bit += 1;
            per_n_div_one_max_val = (1..=(n - bit)).product::<i32>();
        }

        let mut nums = (1..=n).collect::<Vec<_>>();
        let mut list = Vec::with_capacity(n as usize);
        for bit in bit_vec {
            let bit = bit as usize;
            list.push(nums.drain((bit - 1)..bit).next().unwrap() as u8 + b'0');
        }

        String::from_utf8(list).unwrap()
    }
}

#[cfg(test)]
mod sixty_test {
    use super::*;

    #[test]
    fn test_sixty() {
        assert_eq!(Solution::get_permutation(3, 3), "213".to_string());
        assert_eq!(Solution::get_permutation(4, 9), "2314".to_string());
        assert_eq!(Solution::get_permutation(3, 1), "123".to_string());
        assert_eq!(
            Solution::get_permutation(9, 353955),
            "972561438".to_string()
        );
        assert_eq!(
            Solution::get_permutation(9, 296662),
            "839127564".to_string()
        );
    }
}
