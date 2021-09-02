// https://leetcode.com/problems/interleaving-string/

struct Solution;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let (rows, cols, s3len) = (s1.len() + 1, s2.len() + 1, s3.len());
        if rows + cols - 2 != s3len { return false }

        let s1b = s1.as_bytes();
        let s2b = s2.as_bytes();
        let s3b = s3.as_bytes();
        let mut dp = vec![vec![false; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                dp[i][j] = match (i, j) {
                    (0, 0) => true,
                    (0, _) => dp[i][j-1] && (s2b[j-1] == s3b[j-1]),
                    (_, 0) => dp[i-1][j] && (s1b[i-1] == s3b[i-1]),
                    (_, _) => (dp[i-1][j] && (s1b[i-1] == s3b[i+j-1])) ||
                            (dp[i][j-1] && (s2b[j-1] == s3b[i+j-1])),
                }
            }
        }

        dp[rows-1][cols-1]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert!(Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbcbcac".to_string(),
        ));
    }

    #[test]
    fn example2() {
        assert!(!Solution::is_interleave(
            "aabcc".to_string(),
            "dbbca".to_string(),
            "aadbbbaccc".to_string(),
        ));
    }

    #[test]
    fn example3() {
        assert!(Solution::is_interleave(
            "".to_string(),
            "".to_string(),
            "".to_string(),
        ));
    }
}
