// https://leetcode.com/problems/palindrome-permutation/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn can_permute_palindrome(s: String) -> bool {
        s.chars().fold(HashSet::new(), |mut acc, c| {
            if !acc.remove(&c) { acc.insert(c); }
            acc
        }).len() < 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_permute_palindrome() {
        assert!(!Solution::can_permute_palindrome(String::from("code")));
        assert!(Solution::can_permute_palindrome(String::from("aab")));
        assert!(Solution::can_permute_palindrome(String::from("carerac")));
    }
}
