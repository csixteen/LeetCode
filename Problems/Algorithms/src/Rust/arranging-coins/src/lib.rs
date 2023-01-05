// https://leetcode.com/problems/arranging-coins/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn arrange_coins(n: i32) -> i32 {
        (((((1 + 8*(n as i64)) as f64).sqrt() - 1_f64) as i64) / 2) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(2, Solution::arrange_coins(5));
    }

    #[test]
    fn example2() {
        assert_eq!(3, Solution::arrange_coins(8));
    }

    #[test]
    fn example3() {
        assert_eq!(6, Solution::arrange_coins(27));
    }

    #[test]
    fn example4() {
        assert_eq!(60070, Solution::arrange_coins(1804289383));
    }
}
