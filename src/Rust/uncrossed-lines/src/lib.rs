// https://leetcode.com/problems/uncrossed-lines/

struct Solution {}

impl Solution {
    pub fn max_uncrossed_lines(a: Vec<i32>, b: Vec<i32>) -> i32 {
        // This problem is just a variant of the longest common subsequence,
        // which can easily be solved using dynamic programming.
        let mut matrix = vec![vec![0; b.len() + 1]; a.len() + 1];
        
        for j in (0..b.len()).rev() {
            for i in (0..a.len()).rev() {
                if a[i] == b[j] {
                    matrix[i][j] = 1 + matrix[i+1][j+1];
                } else {
                    matrix[i][j] = matrix[i+1][j].max(matrix[i][j+1]);
                }
            }
        }

        matrix[0][0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            2,
            Solution::max_uncrossed_lines(
                vec![1, 4, 2], vec![1, 2, 4]
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            3,
            Solution::max_uncrossed_lines(
                vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2],
            ),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            2,
            Solution::max_uncrossed_lines(
                vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1],
            ),
        );
    }

    #[test]
    fn test_example4() {
        assert_eq!(
            4,
            Solution::max_uncrossed_lines(
                vec![0, 1, 1, 0, 0, 0], vec![0, 0, 1, 1, 0],
            ),
        );
    }

    #[test]
    fn test_example5() {
        assert_eq!(
            4,
            Solution::max_uncrossed_lines(
                vec![0, 0, 1, 1, 0], vec![0, 1, 1, 0, 0, 0],
            ),
        );
    }
}
