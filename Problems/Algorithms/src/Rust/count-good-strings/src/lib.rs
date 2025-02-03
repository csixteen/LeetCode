pub struct Solution;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        let mut dp = vec![0_i64; high as usize + 1];
        let m = 1_000_000_007;
        dp[0] = 1;

        for i in 1..=high as usize {
            if i >= zero as usize {
                dp[i] += dp[i - zero as usize];
            }

            if i >= one as usize {
                dp[i] += dp[i - one as usize];
            }

            dp[i] %= m;
        }

        let mut r = 0;
        (low as usize..=high as usize).for_each(|i| {
            r = (r + dp[i]) % m;
        });

        r as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(8, Solution::count_good_strings(3, 3, 1, 1));
    }

    #[test]
    fn test_case2() {
        assert_eq!(5, Solution::count_good_strings(2, 3, 1, 2));
    }

    #[test]
    fn test_case3() {
        assert_eq!(0, Solution::count_good_strings(2, 5, 6, 7));
    }

    #[test]
    fn test_case4() {
        assert_eq!(764262396, Solution::count_good_strings(200, 200, 10, 1));
    }

    #[test]
    fn test_case5() {
        assert_eq!(215447031, Solution::count_good_strings(1, 100000, 1, 1));
    }
}
