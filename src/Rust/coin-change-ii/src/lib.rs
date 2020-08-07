// https://leetcode.com/problems/coin-change-2/

#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        // This is a typical dynamic programming problem, which
        // can be solved either with a matrix or with a single
        // array with space optimization
        let mut dp: Vec<i32> = vec![0; amount as usize + 1];
        dp[0] = 1; // this is the base case of no amount with zero coins

        for coin in coins.iter() {
            for i in *coin as usize..=amount as usize {
                dp[i] += dp[i - *coin as usize];
            }
        }

        dp[amount as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            4,
            Solution::change(5, vec![1, 2, 5]),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            0,
            Solution::change(3, vec![2]),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            1,
            Solution::change(10, vec![10]),
        );
    }
}
