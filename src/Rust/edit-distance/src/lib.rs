// https://leetcode.com/problems/edit-distance/

struct Solution {}

impl Solution {
    // Recursive version of Levenshtein times out
    fn levenshtein_recursive(a: &[u8], b: &[u8]) -> i32 {
        if a.is_empty() { return b.len() as i32; }
        if b.is_empty() { return a.len() as i32; }
        if a[0] == b[0] { return Self::levenshtein_recursive(&a[1..], &b[1..]); }
        else {
            1 + Self::levenshtein_recursive(a, &b[1..])
                    .min(Self::levenshtein_recursive(&a[1..], b))
                    .min(Self::levenshtein_recursive(&a[1..], &b[1..]))
        }
    }

    fn levenshtein_iterative(a: &[u8], b: &[u8]) -> i32 {
        let (a_len, b_len) = (a.len(), b.len());
        let mut dp = vec![vec![0; b_len + 1]; a_len + 1];

        for i in 1..=a_len { dp[i][0] = i; }
        for j in 1..=b_len { dp[0][j] = j; }

        for j in 1..=b_len {
            for i in 1..=a_len {
                let cost = if a[i-1] == b[j-1] { 0 } else { 1 };
                dp[i][j] = (dp[i-1][j] + 1)
                                .min(dp[i][j-1] + 1)
                                .min(dp[i-1][j-1] + cost);
            }
        }

        dp[a_len][b_len] as i32
    }

    pub fn min_distance(word1: String, word2: String) -> i32 {
        Self::levenshtein_iterative(word1.as_bytes(), word2.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            3,
            Solution::min_distance(
                String::from("horse"),
                String::from("ros")
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            5,
            Solution::min_distance(
                String::from("intention"),
                String::from("execution"),
            ),
        );
    }
}
