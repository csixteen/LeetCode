// https://leetcode.com/problems/valid-parentheses/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        fn opening(c: char) -> bool {
            match c {
                '(' | '[' | '{' => true,
                _ => false,
            }
        }

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
            match opening(c) {
                true => stack.push(c),
                false if stack.is_empty() => return false,
                false if stack[stack.len() - 1] != matching(c) => return false,
                _ => { stack.pop(); },
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
        assert!(
            Solution::is_valid(String::from("()")),
        );
    }

    #[test]
    fn test_example2() {
        assert!(
            Solution::is_valid(String::from("()[]{}")),
        );
    }

    #[test]
    fn test_example3() {
        assert!(
            !Solution::is_valid(String::from("(]")),
        );
    }

    #[test]
    fn test_example4() {
        assert!(
            !Solution::is_valid(String::from("([)]")),
        );
    }

    #[test]
    fn test_example5() {
        assert!(
            Solution::is_valid(String::from("{[]}")),
        );
    }

    #[test]
    fn test_example6() {
        assert!(
            !Solution::is_valid(String::from("(")),
        );
    }
}
