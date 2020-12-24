// https://leetcode.com/problems/word-search/

#![allow(dead_code)]

struct Solution;

impl Solution {
    fn go(board: &mut Vec<Vec<char>>, i: usize, j: usize, w: &[u8]) -> bool {
        if w.len() == 0 { return true }
        else if 
            i >= board.len() || 
            j >= board[0].len() || 
            board[i][j] as u8 != w[0] {
                return false
        }

        board[i][j] = '#';

        let ret = Self::go(board, i.wrapping_add(1), j, &w[1..]) ||
                  Self::go(board, i.wrapping_sub(1), j, &w[1..]) ||
                  Self::go(board, i, j.wrapping_add(1), &w[1..]) ||
                  Self::go(board, i, j.wrapping_sub(1), &w[1..]);

        board[i][j] = w[0] as char;
        ret
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let mut board = board;
        let rows = board.len();
        let cols = board[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if Self::go(&mut board, i, j, word.as_bytes()) { return true }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exist() {
        let board = vec![
            vec!['A', 'B', 'C', 'E'],
            vec!['S', 'F', 'C', 'S'],
            vec!['A', 'D', 'E', 'E'],
        ];

        assert!(Solution::exist(board.clone(), String::from("ABCCED")));
        assert!(Solution::exist(board.clone(), String::from("SEE")));
        assert!(!Solution::exist(board.clone(), String::from("ABCB")));
    }
}
