// https://leetcode.com/problems/count-square-submatrices-with-all-ones/

struct Solution {}

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let (rows, cols) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; cols + 1]; rows + 1];
        let mut total = 0;

        for i in 1..=rows {
            for j in 1..=cols {
                if matrix[i-1][j-1] == 1 {
                    dp[i][j] = 1 + dp[i][j-1].min(dp[i-1][j-1]).min(dp[i-1][j]);
                    total += dp[i][j];
                }
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            15,
            Solution::count_squares(
                vec![
                    vec![0, 1, 1, 1],
                    vec![1, 1, 1, 1],
                    vec![0, 1, 1, 1],
                ],
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            7,
            Solution::count_squares(
                vec![
                    vec![1, 0, 1],
                    vec![1, 1, 0],
                    vec![1, 1, 0],
                ],
            ),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            20,
            Solution::count_squares(
                vec![
                    vec![0, 1, 1, 1, 0],
                    vec![0, 1, 1, 1, 0],
                    vec![0, 1, 1, 1, 1],
                    vec![0, 1, 0, 1, 1],
                    vec![1, 0, 0, 0, 0],
                ],
            ),
        );
    }
}
