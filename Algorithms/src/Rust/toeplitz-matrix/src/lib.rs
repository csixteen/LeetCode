// https://leetcode.com/problems/toeplitz-matrix/

struct Solution;

impl Solution {
    pub fn is_toeplitz_matrix(matrix: Vec<Vec<i32>>) -> bool {
        let (rows, cols) = (matrix.len(), matrix[0].len());

        for j in 1..rows {
            for i in 1..cols {
                if matrix[j][i] != matrix[j-1][i-1] {
                    return false
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert!(
            Solution::is_toeplitz_matrix(
                vec![
                    vec![1, 2, 3, 4],
                    vec![5, 1, 2, 3],
                    vec![9, 5, 1, 2],
                ],
            ),
        );
    }

    #[test]
    fn example2() {
        assert!(
            !Solution::is_toeplitz_matrix(
                vec![vec![1, 2], vec![2, 2]],
            ),
        );
    }
}
