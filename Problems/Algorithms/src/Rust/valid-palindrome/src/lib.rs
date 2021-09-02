// https://leetcode.com/problems/valid-palindrome/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let f: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|x| x.is_alphanumeric())
            .collect();

        f == f.iter().rev().cloned().collect::<Vec<char>>()
    }

    pub fn is_palindrome2(s: String) -> bool {
        let s: Vec<char> = s.to_lowercase().chars().filter(|x| x.is_alphanumeric()).collect();
        let (mut lo, mut hi) = (0, s.len() - 1);

        if s.is_empty() { return true; }

        while lo < hi {
            if s[lo] != s[hi] { return false; }
            lo += 1;
            hi -= 1;
        }

        true
    }

    pub fn is_palindrome3(s: String) -> bool {
        let i = s
            .chars()
            .filter(|x| x.is_alphanumeric())
            .map(|x| x.to_ascii_lowercase());

        i.clone().eq(i.rev())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        assert!(
            Solution::is_palindrome3(String::from("A man, a plan, a canal: Panama")),
        );
    }

    #[test]
    fn test_example2() {
        assert!(
            !Solution::is_palindrome3(String::from("race a car")),
        );
    }

    #[test]
    fn test_example3() {
        assert!(Solution::is_palindrome3(String::new()));
    }

    #[test]
    fn test_example4() {
        assert!(!Solution::is_palindrome3(String::from("0P")));
    }
}
