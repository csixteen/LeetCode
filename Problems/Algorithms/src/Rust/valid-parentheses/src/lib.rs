// https://leetcode.com/problems/valid-parentheses/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        #[inline]
        fn matching(c: char) -> char {
            match c {
                ')' => '(',
                ']' => '[',
                '}' => '{',
                _ => panic!(),
            }
        }

        let mut stack = Vec::new();

        for c in s.chars() {
            if c == '(' || c == '[' || c == '{' {
                stack.push(c);
            } else if Some(matching(c)) != stack.pop() {
                return false;
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert!(Solution::is_valid(String::from("()")),);
    }

    #[test]
    fn test_example2() {
        assert!(Solution::is_valid(String::from("()[]{}")),);
    }

    #[test]
    fn test_example3() {
        assert!(!Solution::is_valid(String::from("(]")),);
    }

    #[test]
    fn test_example4() {
        assert!(!Solution::is_valid(String::from("([)]")),);
    }

    #[test]
    fn test_example5() {
        assert!(Solution::is_valid(String::from("{[]}")),);
    }

    #[test]
    fn test_example6() {
        assert!(!Solution::is_valid(String::from("(")),);
    }
}
