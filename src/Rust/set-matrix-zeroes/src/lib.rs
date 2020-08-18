// https://leetcode.com/problems/set-matrix-zeroes/

struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut rows = vec![false; matrix.len()];
        let mut cols = vec![false; matrix[0].len()];

        for row in 0..matrix.len() {
            for col in 0..matrix[0].len() {
                if matrix[row][col] == 0 {
                    rows[row] = true;
                    cols[col] = true;
                }
            }
        }

        for (row, is_set) in rows.iter().enumerate() {
            if *is_set {
                for col in 0..matrix[0].len() {
                    matrix[row][col] = 0;
                }
            }
        }

        for (col, is_set) in cols.iter().enumerate() {
            if *is_set {
                for row in 0..matrix.len() {
                    matrix[row][col] = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut matrix = vec![
            vec![0, 1, 2, 0],
            vec![3, 4, 5, 2],
            vec![1, 3, 1, 5],
        ];

        Solution::set_zeroes(&mut matrix);

        assert_eq!(
            matrix,
            vec![
                vec![0, 0, 0, 0],
                vec![0, 4, 5, 0],
                vec![0, 3, 1, 0],
            ],
        );
    }
}
