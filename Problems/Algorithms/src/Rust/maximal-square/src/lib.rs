// https://leetcode.com/problems/maximal-square/

#![allow(dead_code)]

use std::cmp::max;

struct Solution;

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut max_side = 0;
        let mut dp = vec![vec![0; cols + 1]; rows + 1];

        for j in (0..cols).rev() {
            for i in (0..rows).rev() {
                if matrix[i][j] == '1' {
                    dp[i][j] = 1 + dp[i][j+1].min(dp[i+1][j]).min(dp[i+1][j+1]);
                    max_side = max(max_side, dp[i][j]);
                }
            }
        }

        max_side as i32 * max_side as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        matrix: Vec<Vec<char>>,
        expected: i32,
    }

    #[test]
    fn test_maximal_square() {
        let test_cases = [
            TestCase {
                matrix: Vec::new(),
                expected: 0,
            },
            TestCase {
                matrix: vec![
                    vec!['1', '0', '1', '0', '0'],
                    vec!['1', '0', '1', '1', '1'],
                    vec!['1', '1', '1', '1', '1'],
                    vec!['1', '0', '0', '1', '0'],
                ],
                expected: 4,
            },
            TestCase {
                matrix: vec![
                    vec!['0', '1', '1', '1', '0'],
                    vec!['0', '1', '1', '1', '0'],
                    vec!['0', '1', '1', '1', '1'],
                    vec!['0', '1', '0', '1', '1'],
                    vec!['1', '0', '0', '0', '0'],
                ],
                expected: 9,
            }
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::maximal_square(case.matrix.to_vec()),
            );
        }
    }
}
