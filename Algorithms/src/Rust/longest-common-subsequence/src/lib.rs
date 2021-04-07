// https://leetcode.com/problems/longest-common-subsequence/

#![allow(dead_code)]

use std::cmp;

struct Solution; 

impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut matrix = vec![vec![0; text2.len() + 1]; text1.len() + 1];
        let text1_chars: Vec<char> = text1.chars().collect();
        let text2_chars: Vec<char> = text2.chars().collect();

        for j in (0..text2.len()).rev() {
            for i in (0..text1.len()).rev() {
                if text1_chars[i] == text2_chars[j] {
                    matrix[i][j] = 1 + matrix[i+1][j+1];
                } else {
                    matrix[i][j] = cmp::max(matrix[i+1][j], matrix[i][j+1]);
                }
            }
        }

        return matrix[0][0];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        text1: String,
        text2: String,
        expected: i32,
    }

    #[test]
    fn test_longest_common_subsequence() {
        let test_cases = [
            TestCase {
                text1: String::from("abcde"),
                text2: String::from("ace"),
                expected: 3,
            },
            TestCase {
                text1: String::from("nmacebafe"),
                text2: String::from("ace"),
                expected: 3,
            },
            TestCase {
                text1: String::from("abc"),
                text2: String::from("abc"),
                expected: 3,
            },
            TestCase {
                text1: String::from("abc"),
                text2: String::from("def"),
                expected: 0,
            }
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::longest_common_subsequence(case.text1.clone(), case.text2.clone()),
            );
        }
    }
}
