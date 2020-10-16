// https://leetcode.com/problems/generate-parentheses/

#![allow(dead_code)]

struct Solution;

impl Solution {
    fn parenthesis(acc: &mut Vec<String>, curr: String, left: i32, right: i32) {
        if left == 0 && right == 0 {
            acc.push(curr);
        } else {
            if left > 0 {
                Self::parenthesis(acc, format!("{}(", curr), left-1, right);
            }

            if right > left {
                Self::parenthesis(acc, format!("{})", curr), left, right-1);
            }
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut acc = Vec::<String>::new();

        Solution::parenthesis(&mut acc, String::from(""), n, n);
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis_1() {
        assert_eq!(
            vec![
                String::from("()"),
            ],
            Solution::generate_parenthesis(1),
        );
    }

    #[test]
    fn test_generate_parenthesis_3() {
        assert_eq!(
            vec![
                String::from("((()))"),
                String::from("(()())"),
                String::from("(())()"),
                String::from("()(())"),
                String::from("()()()"),
            ],
            Solution::generate_parenthesis(3),
        );
    }
}
