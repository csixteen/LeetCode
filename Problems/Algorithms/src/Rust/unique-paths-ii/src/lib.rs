// https://leetcode.com/problems/unique-paths-ii/

struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles2(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (R, C) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![vec![0; C + 1]; R + 1];

        dp[1][1] = 1;

        for row in 1..=R {
            for col in 1..=C {
                if obstacle_grid[row-1][col-1] == 1 {
                    dp[row][col] = 0;
                } else {
                    dp[row][col] = dp[row][col].max(dp[row-1][col] + dp[row][col-1]);
                }
            }
        }

        dp[R][C]
    }

    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (R, C) = (obstacle_grid.len(), obstacle_grid[0].len());
        if obstacle_grid[0][0] == 1 || obstacle_grid[R-1][C-1] == 1 { return 0; }

        let mut dp = vec![0; C + 1];
        dp[C-1] = 1;
        for row in (0..R).rev() {
            for col in (0..C).rev() {
                if obstacle_grid[row][col] == 1 {
                    dp[col] = 0;
                } else {
                    dp[col] += dp[col+1];
                }
            }
        }

        dp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            2,
            Solution::unique_paths_with_obstacles(
                vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]
            ),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            1,
            Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]]),
        );
    }
}
