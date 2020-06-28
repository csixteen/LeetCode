// https://leetcode.com/problems/rotate-image/

struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let len = matrix.len();

        for row in 0..(len/2) {
            for col in 0..(len + 1)/2 {
                let curr = matrix[row][col];

                matrix[row][col] = matrix[len - col - 1][row];
                matrix[len - col - 1][row] = matrix[len - row - 1][len - col - 1];
                matrix[len - row - 1][len - col - 1] = matrix[col][len - row - 1];
                matrix[col][len - row - 1] = curr;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut m = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];

        Solution::rotate(&mut m);

        assert_eq!(
            m,
            vec![
                vec![7, 4, 1],
                vec![8, 5, 2],
                vec![9, 6, 3],
            ],
        );
    }

    #[test]
    fn test_exammple2() {
        let mut m = vec![
            vec![ 5, 1, 9,11],
            vec![ 2, 4, 8,10],
            vec![13, 3, 6, 7],
            vec![15,14,12,16],
        ];

        Solution::rotate(&mut m);

        assert_eq!(
            m,
            vec![
                [15,13, 2, 5],
                [14, 3, 4, 1],
                [12, 6, 8, 9],
                [16, 7,10,11],
            ],
        );
    }
}
