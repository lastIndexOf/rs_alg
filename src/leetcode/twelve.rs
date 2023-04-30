pub struct Solution;

// I 1
// V 5
// X 10
// L 50
// C 100
// D 500
// M 1000

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let m_len = num / 1000;
        let num = num % 1000;
        let cm_len = num / 900;
        let num = num % 900;
        let d_len = num / 500;
        let num = num % 500;
        let cd_len = num / 400;
        let num = num % 400;
        let c_len = num / 100;
        let num = num % 100;
        let xc_len = num / 90;
        let num = num % 90;
        let l_len = num / 50;
        let num = num % 50;
        let xl_len = num / 40;
        let num = num % 40;
        let x_len = num / 10;
        let num = num % 10;
        let ix_len = num / 9;
        let num = num % 9;
        let v_len = num / 5;
        let num = num % 5;
        let iv_len = num / 4;
        let num = num % 4;
        let i_len = num;

        println!(
            "m_len = {m_len}, cm_len = {cm_len}, d_len = {d_len}, cd_len = {cd_len}, c_len = {c_len}, xc_len = {xc_len}, l_len = {l_len}, xl_len = {xl_len}, x_len = {x_len}, ix_len = {ix_len}, v_len = {v_len}, iv_len = {iv_len}, i_len = {i_len}"
        );
        (0..m_len)
            .map(|_| "M")
            .chain((0..cm_len).map(|_| "CM"))
            .chain((0..d_len).map(|_| "D"))
            .chain((0..cd_len).map(|_| "CD"))
            .chain((0..c_len).map(|_| "C"))
            .chain((0..xc_len).map(|_| "XC"))
            .chain((0..l_len).map(|_| "L"))
            .chain((0..xl_len).map(|_| "XL"))
            .chain((0..x_len).map(|_| "X"))
            .chain((0..ix_len).map(|_| "IX"))
            .chain((0..v_len).map(|_| "V"))
            .chain((0..iv_len).map(|_| "IV"))
            .chain((0..i_len).map(|_| "I"))
            .collect::<String>()
    }
}

#[cfg(test)]
mod twelve_test {
    use super::*;

    #[test]
    fn test_twelve() {
        assert_eq!(Solution::int_to_roman(3), "III".to_string());
        assert_eq!(Solution::int_to_roman(4), "IV".to_string());
        assert_eq!(Solution::int_to_roman(9), "IX".to_string());
        assert_eq!(Solution::int_to_roman(58), "LVIII".to_string());
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV".to_string());
    }
}
