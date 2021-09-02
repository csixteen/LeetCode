// https://leetcode.com/problems/island-perimeter/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn island_perimeter2(grid: Vec<Vec<i32>>) -> i32 {
        let mut perimeter = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 1 {
                    perimeter += 4;

                    if row > 0 && grid[row-1][col] == 1 {
                        perimeter -= 2;
                    }

                    if col > 0 && grid[row][col-1] == 1 {
                        perimeter -= 2;
                    }
                }
            }
        }

        perimeter
    }

    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        fn dfs(g: &Vec<Vec<i32>>, r: i32, c: i32, v: &mut HashSet<(i32, i32)>, acc: &mut i32) {
            if r >= 0 && r < (g.len() as i32) && c >= 0 && c < (g[0].len() as i32) && g[r as usize][c as usize] == 1 && !v.contains(&(r, c)) {
                v.insert((r, c));
                *acc += 4;
                
                if r > 0 && g[(r-1) as usize][c as usize] == 1 { *acc -= 2; }
                if c > 0 && g[r as usize][(c-1) as usize] == 1 { *acc -= 2; }

                dfs(g, r-1, c, v, acc);
                dfs(g, r+1, c, v, acc);
                dfs(g, r, c-1, v, acc);
                dfs(g, r, c+1, v, acc);
            }
        }

        let mut ret = 0;
        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if grid[row][col] == 1 {
                    dfs(&grid, row as i32, col as i32, &mut HashSet::new(),  &mut ret);
                    return ret
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_island_perimeter() {
        assert_eq!(
            16,
            Solution::island_perimeter(
                vec![vec![0, 1, 0, 0], vec![1, 1, 1, 0], vec![0, 1, 0, 0], vec![1, 1, 0, 0]]
            )
        );

        assert_eq!(
            4,
            Solution::island_perimeter(vec![vec![1]])
        );

        assert_eq!(
            4,
            Solution::island_perimeter(vec![vec![1, 0]])
        );
    }
}
