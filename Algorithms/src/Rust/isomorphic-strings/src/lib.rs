// https://leetcode.com/problems/isomorphic-strings/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut m1: HashMap<char, usize> = HashMap::new();
        let mut m2: HashMap<char, usize> = HashMap::new();

        for (i, (ss, tt)) in s.chars().zip(t.chars()).enumerate() {
            if m1.insert(ss, i) != m2.insert(tt, i) {
                return false
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert!(Solution::is_isomorphic("egg".to_string(), "add".to_string()));
    }

    #[test]
    fn test_example2() {
        assert!(!Solution::is_isomorphic("foo".to_string(), "bar".to_string()));
    }

    #[test]
    fn test_example3() {
        assert!(Solution::is_isomorphic("paper".to_string(), "title".to_string()));
    }

    #[test]
    fn test_example4() {
        assert!(!Solution::is_isomorphic("aa".to_string(), "ab".to_string()));
    }

    #[test]
    fn test_exampler54() {
        assert!(!Solution::is_isomorphic("ab".to_string(), "aa".to_string()));
    }
}
