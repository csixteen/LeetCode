// https://leetcode.com/problems/spiral-matrix-ii/

struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut res = vec![vec![0_i32; n]; n];

        let dr = [0, 1, 0, -1];
        let dc = [1, 0, -1, 0];

        let (mut r, mut c) = (0_i32, 0_i32);
        let mut di = 0;

        let mut i = 1;
        while (i as usize) <= n*n {
            res[r as usize][c as usize] = i;
            i += 1;

            let new_r = (r + dr[di]) as usize;
            let new_c = (c + dc[di]) as usize;

            if 0 <= new_r && new_r < n && 0 <= new_c  && new_c < n && res[new_r][new_c] == 0 {
                r = new_r as i32;
                c = new_c as i32;
            } else {
                di = (di + 1) % 4;
                r += dr[di];
                c += dc[di];
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            vec![vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]],
            Solution::generate_matrix(3),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            vec![vec![1]],
            Solution::generate_matrix(1),
        );
    }
}
