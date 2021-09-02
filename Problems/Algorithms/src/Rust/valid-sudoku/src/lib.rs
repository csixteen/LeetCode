// https://leetcode.com/problems/valid-sudoku/

use std::collections::HashSet;

struct Solution; 

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut verticals: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut horizontals: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut squares: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        for row in 0..9 {
            for col in 0..9 {
                if board[row][col] != '.' && (
                    verticals[col].contains(&board[row][col]) ||
                    horizontals[row].contains(&board[row][col]) ||
                    squares[(row / 3) * 3 + col / 3].contains(&board[row][col])) {
                    return false;
                } else {
                    verticals[col].insert(board[row][col]);
                    horizontals[row].insert(board[row][col]);
                    squares[(row / 3) * 3 + col / 3].insert(board[row][col]);
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert!(
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
        );
    }

    #[test]
    fn test_example2() {
        assert!(
            !Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ]),
        );
    }
}
