// https://leetcode.com/problems/decode-ways/

struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        let chars = s.as_bytes();
        let mut dp = vec![0; s.len() + 1];

        dp[0] = 1;
        dp[1] = if chars[0] == b'0' { 0 } else { 1 };

        for i in 2..dp.len() {
            if chars[i-1] != b'0' {
                dp[i] = dp[i-1];
            }

            let n = (chars[i-2] - b'0') * 10 + (chars[i-1] - b'0');
            if n >= 10 && n <= 26 {
                dp[i] += dp[i-2];
            }
        }

        dp[s.len()]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(2, Solution::num_decodings("12".to_string()));
    }

    #[test]
    fn example2() {
        assert_eq!(3, Solution::num_decodings("226".to_string()));
    }

    #[test]
    fn example3() {
        assert_eq!(0, Solution::num_decodings("0".to_string()));
    }

    #[test]
    fn example4() {
        assert_eq!(0, Solution::num_decodings("06".to_string()));
    }

    #[test]
    fn example5() {
        assert_eq!(0, Solution::num_decodings("60".to_string()));
    }
}
