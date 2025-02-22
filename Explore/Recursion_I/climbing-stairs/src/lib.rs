pub struct Solution {}

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![0; n + 2];

        dp[1] = 1;
        dp[2] = 2;

        (3..=n).for_each(|i| dp[i] = dp[i - 1] + dp[i - 2]);

        dp[n]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn test_case2() {
        assert_eq!(3, Solution::climb_stairs(3));
    }

    #[test]
    fn test_case3() {
        assert_eq!(1, Solution::climb_stairs(1));
    }
}
