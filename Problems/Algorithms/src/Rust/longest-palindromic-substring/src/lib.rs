// https://leetcode.com/problems/longest-palindromic-substring/

#![allow(dead_code)]

struct Solution;

impl Solution {
    fn expand(s: &Vec<char>, left: usize, right: usize) -> usize {
        let (mut l, mut r) = (left as i32, right as i32);

        while l >= 0 && r < s.len() as i32 && s[l as usize] == s[r as usize] {
            l -= 1;
            r += 1;
        }

        (r - l - 1) as usize
    }

    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() { return "".to_string() }

        let (mut start, mut end) = (0, 0);
        let s: Vec<_> = s.chars().collect();

        for i in 0..s.len() {
            let len1 = Self::expand(&s, i, i);
            let len2 = Self::expand(&s, i, i + 1);
            let len = len1.max(len2);

            if len > end - start + 1 {
                start = i - (len - 1) / 2;
                end = i + len / 2;
            }
        }

        s[start..=end].into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            "bab".to_string(),
            Solution::longest_palindrome("babad".to_string()),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            "bb".to_string(),
            Solution::longest_palindrome("cbbd".to_string()),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            "aaaaaaaa".to_string(),
            Solution::longest_palindrome("aaaaaaaa".to_string()),
        );
    }

    #[test]
    fn test_example4() {
        assert_eq!(
            "b".to_string(),
            Solution::longest_palindrome("b".to_string()),
        );
    }
}
