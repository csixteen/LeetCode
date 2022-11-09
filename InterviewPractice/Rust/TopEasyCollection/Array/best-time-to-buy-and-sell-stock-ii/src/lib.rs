struct Solution;

impl Solution {
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        (1..prices.len()).fold(0, |acc, i| acc + (prices[i] - prices[i-1]).max(0))
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.windows(2).fold(0, |acc, w| acc + (w[1] - w[0]).max(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(7, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    }

    #[test]
    fn example2() {
        assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
    }

    #[test]
    fn example3() {
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
    }
}
