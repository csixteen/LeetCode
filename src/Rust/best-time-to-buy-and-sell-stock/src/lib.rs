
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut global = 0;
        let mut lowest = i32::MAX;

        for p in prices.iter() {
            lowest = lowest.min(*p);
            global = global.max(*p - lowest);
        }

        global
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    }

    #[test]
    fn test_example2() {
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }

    #[test]
    fn test_example3() {
        assert_eq!(17, Solution::max_profit(vec![3, 19, 4, 20, 1, 10]));
    }
}
