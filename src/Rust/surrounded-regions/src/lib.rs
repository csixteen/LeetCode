// https://leetcode.com/problems/surrounded-regions/

#![allow(non_snake_case)]
#![allow(dead_code)]

struct Solution {}

impl Solution {
    fn DFS(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i >= board.len() || j >= board[0].len() || board[i][j] != 'O' {
            return ()
        }

        board[i][j] = 'Y';

        Self::DFS(board, i-1, j);
        Self::DFS(board, i+1, j);
        Self::DFS(board, i, j-1);
        Self::DFS(board, i, j+1);
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() { return () }

        // Start with top and bottom borders
        for row in [0, board.len() - 1].iter() {
            for col in 0..board[0].len() {
                if board[*row][col] == 'O' {
                    Self::DFS(board, *row, col);
                }
            }
        }

        // Then left and right borders
        for row in 0..board.len() {
            for col in [0, board[0].len() - 1].iter() {
                if board[row][*col] == 'O' {
                    Self::DFS(board, row, *col);
                }
            }
        }

        for row in 0..board.len() {
            for col in 0..board[0].len() {
                if board[row][col] == 'O' { board[row][col] = 'X'; }
                else if board[row][col] == 'Y' { board[row][col] = 'O'; }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut input = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];

        Solution::solve(&mut input);

        assert_eq!(
            input,
            vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ],
        );
    }
}
