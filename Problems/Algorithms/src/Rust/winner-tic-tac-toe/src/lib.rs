// https://leetcode.com/problems/find-winner-on-a-tic-tac-toe-game/

struct Solution;

impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        fn winner(grid: &Vec<Vec<String>>) -> Option<&String> {
            for row in 0..3 {
                if !grid[row][0].is_empty() && grid[row][0] == grid[row][1] && grid[row][0] == grid[row][2] {
                    return Some(&grid[row][0]);
                }
            }

            for col in 0..3 {
                if !grid[0][col].is_empty() && grid[0][col] == grid[1][col] && grid[0][col] == grid[2][col] {
                    return Some(&grid[0][col]);
                }
            }

            if !grid[0][0].is_empty() && grid[0][0] == grid[1][1] && grid[0][0] == grid[2][2] {
                return Some(&grid[0][0]);
            }

            if !grid[0][2].is_empty() && grid[0][2] == grid[1][1] && grid[0][2] == grid[2][0] {
                return Some(&grid[0][2]);
            }

            None
        }

        let g = moves
            .iter()
            .zip(["A".to_string(), "B".to_string()].iter().cycle())
            .fold(vec![vec![String::new(); 3]; 3], |mut acc, (pos, p)| {
                acc[pos[0] as usize][pos[1] as usize] = p.to_string();
                acc
            });

        if let Some(w) = winner(&g) {
            w.to_string()
        } else if moves.len() < 9 {
            String::from("Pending")
        } else {
            String::from("Draw")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tictactoe() {
        assert_eq!(
            String::from("A"),
            Solution::tictactoe(
                vec![vec![0, 0], vec![2, 0], vec![1, 1], vec![2, 1], vec![2, 2]]
            )
        );

        assert_eq!(
            String::from("B"),
            Solution::tictactoe(
                vec![vec![0, 0], vec![1, 1], vec![0, 1], vec![0, 2], vec![1, 0], vec![2, 0]]
            )
        );

        assert_eq!(
            String::from("Draw"),
            Solution::tictactoe(
                vec![
                    vec![0, 0],
                    vec![1, 1],
                    vec![2, 0],
                    vec![1, 0],
                    vec![1, 2],
                    vec![2, 1],
                    vec![0, 1],
                    vec![0, 2],
                    vec![2, 2]
                ]
            )
        );

        assert_eq!(
            String::from("Pending"),
            Solution::tictactoe(vec![vec![0, 0], vec![1, 1]])
        );
    }
}
