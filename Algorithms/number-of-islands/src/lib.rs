struct Solution {}

impl Solution {
    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] == '0' {
            return ();
        }

        grid[i][j] = '0';

        Self::dfs(grid, i.wrapping_sub(1), j);
        Self::dfs(grid, i, j.wrapping_sub(1));
        Self::dfs(grid, i+1, j);
        Self::dfs(grid, i, j+1);
    }

    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut islands = 0;
        let num_rows = grid.len();

        if num_rows == 0 {
            return 0;
        }

        let num_cols = grid[0].len();

        for i in 0..num_rows {
            for j in 0..num_cols {
                if grid[i][j] == '1' {
                    islands += 1;

                    Self::dfs(&mut grid, i, j);
                }
            }
        }

        islands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        grid: Vec<Vec<char>>,
        expected: i32,
    }

    #[test]
    fn test_num_islands() {
        let test_cases = [
            TestCase { grid: Vec::new(), expected: 0 },
            TestCase { 
                grid: vec![vec!['0', '0'], vec!['0', '0']],
                expected: 0,
            },
            TestCase {
                grid: vec![vec!['1', '1'], vec!['1', '1']],
                expected: 1,
            },
            TestCase {
                grid: vec![vec!['1', '0'], vec!['0', '1']],
                expected: 2,
            },
            TestCase {
                grid: vec![
                    vec!['1', '1', '1'],
                    vec!['0', '1', '0'],
                    vec!['1', '1', '1'],
                ],
                expected: 1,
            },
            TestCase {
                grid: vec![
                    vec!['1', '1', '1', '1', '0'],
                    vec!['1', '1', '0', '1', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '0', '0', '0'],
                ],
                expected: 1,
            },
            TestCase {
                grid: vec![
                    vec!['1', '1', '0', '0', '0'],
                    vec!['1', '1', '0', '0', '0'],
                    vec!['0', '0', '1', '0', '0'],
                    vec!['0', '0', '0', '1', '1'],
                ],
                expected: 3,
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::num_islands(case.grid.clone()),
            );
        }
    }
}
