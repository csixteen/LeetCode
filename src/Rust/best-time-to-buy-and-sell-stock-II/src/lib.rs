// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/

struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices
            .iter()
            .enumerate()
            .skip(1)
            .fold(0, |acc, (i, p)| {
                acc + (p - prices[i-1]).max(0)
            })
    }

    // This one actually takes less memory than the previous one
    pub fn max_profit_no_iterators(prices: Vec<i32>) -> i32 {
        let mut m: i32 = 0;

        for i in 1..prices.len() {
            m += (prices[i] - prices[i-1]).max(0)
        }

        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            7,
            Solution::max_profit(vec![7, 1, 5, 3, 6, 4]),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            4,
            Solution::max_profit(vec![1, 2, 3, 4, 5]),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            0,
            Solution::max_profit(vec![7, 6, 4, 3, 1]),
        );
    }
}
