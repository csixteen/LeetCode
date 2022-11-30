use std::collections::HashSet;

struct Solution;

impl Solution {
    fn valid_row(row: &Vec<char>) -> bool {
        let mut seen: HashSet<char> = HashSet::new();
        for c in row.iter() {
            if *c != '.' && seen.contains(&c) {
                return false
            }
            seen.insert(*c);
        }
        true
    }

    fn valid_col(col: usize, board: &Vec<Vec<char>>) -> bool {
        let mut seen: HashSet<char> = HashSet::new();
        for row in 0..board.len() {
            let c = board[row][col];
            if c != '.' && seen.contains(&c) {
                return false
            }
            seen.insert(c);
        }
        true
    }

    fn valid_square(x: usize, y: usize, board: &Vec<Vec<char>>) -> bool {
        let mut seen: HashSet<char> = HashSet::new();
        for (i, j) in [(0, 0), (0, 1), (0, 2), (1, 0), (1, 1), (1, 2), (2, 0), (2, 1), (2, 2)] {
            let c = board[x+i][y+j];
            if c != '.' && seen.contains(&c) {
                return false
            }
            seen.insert(c);
        }
        true
    }

    pub fn is_valid_sudoku2(board: Vec<Vec<char>>) -> bool {
        let rows = (0..9).fold(true, |acc, row| acc && Solution::valid_row(&board[row]));
        let cols = (0..9).fold(true, |acc, col| acc && Solution::valid_col(col, &board));
        let sqrs = (0..9).fold(true, |acc, i| {
            let (x, y) = (i % 3, i / 3);
            acc && Solution::valid_square(x*3, y*3, &board)
        });
        rows && cols && sqrs
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut seen_rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut seen_cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut seen_diag: Vec<HashSet<char>> = vec![HashSet::new(); 9];

        for row in 0..9 {
            for col in 0..9 {
                let c = board[row][col];
                if c != '.' && (
                    !seen_rows[row].insert(c) ||
                    !seen_cols[col].insert(c) ||
                    !seen_diag[(row / 3) * 3 + col / 3].insert(c)) {
                    return false
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
    fn test_valid_row() {
        assert!(Solution::valid_row(&vec!['.', '.', '.']));
        assert!(!Solution::valid_row(&vec!['1', '1', '.']));
        assert!(Solution::valid_row(&vec!['1', '2', '3']));
    }

    #[test]
    fn test_valid_col() {
        assert!(Solution::valid_col(0, &vec![vec!['.'], vec!['.']]));
        assert!(Solution::valid_col(0, &vec![vec!['1'], vec!['2']]));
        assert!(!Solution::valid_col(0, &vec![vec!['1'], vec!['1']]));
    }

    #[test]
    fn example1() {
        assert!(
            Solution::is_valid_sudoku(
                vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
                ]
            )
        );
    }

    #[test]
    fn example2() {
        assert!(
            !Solution::is_valid_sudoku(
                vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
                ]
            )
        );
    }
}
