// https://leetcode.com/problems/spiral-matrix/

struct Solution;

impl Solution {
    // Initial version, just to pass the submission. Complete garbage, but
    // it runs in 0ms and is accepted.
    pub fn spiral_order3(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut a = (0, 0);
        let mut b = (0, matrix[0].len() - 1);
        let mut c = (matrix.len() - 1, matrix[0].len() - 1);
        let mut d = (matrix.len() - 1, 0);

        let mut res = Vec::new();
        let mut total = matrix.len() * matrix[0].len();

        loop {
            // A -> B
            res.append(&mut matrix[a.0].iter().skip(a.1).take(b.1 + 1 - a.1).cloned().collect());
            total -= (b.1 + 1) - a.1;
            if total <= 0 { break; }
            a = (a.0 + 1, a.1 + 1);

            // B -> C
            res.append(&mut matrix.iter().skip(b.0 + 1).take(c.0-b.0).map(|r| r[b.1]).collect());
            total -= c.0 - b.0;
            if total <= 0 { break; }
            b = (b.0 + 1, b.1 - 1);

            // C -> D
            res.append(&mut matrix[c.0].iter().rev().skip(d.1 + 1).take(c.1 - d.1).cloned().collect());
            total -= c.1 - d.1;
            if total <= 0 { break; }
            c = (c.0 - 1, c.1 - 1);

            // D -> A
            res.append(&mut matrix.iter().rev().skip(a.0).take(d.0 - a.0).map(|r| r[d.1]).collect());
            total -= d.0 - a.0;
            if total <= 0 { break; }
            d = (d.0 - 1, d.1 + 1);
        }

        res
    }

    pub fn spiral_order2(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (R, C) = (matrix.len(), matrix[0].len());

        // Delta for rows and columns based on the 4 directions:
        // right -> down -> left -> up
        let dr = [0, 1, 0, -1];
        let dc = [1, 0, -1, 0];

        let mut res = Vec::new();
        let mut visited = vec![vec![false; C]; R];

        let (mut r, mut c) = (0_i32, 0_i32);
        let mut di = 0;  // Initial direction: right

        for i in 0..(R * C) {
            res.push(matrix[r as usize][c as usize]);
            visited[r as usize][c as usize] = true;

            let new_r = (r + dr[di]) as usize;
            let new_c = (c + dc[di]) as usize;

            if new_r >= 0 && new_r < R && new_c >= 0 && new_c < C && !visited[new_r][new_c] {
                r = new_r as i32;
                c = new_c as i32;
            } else {
                // We're either out of bounds or the next cell was already visited.
                // In this case, we'll change direction immediately.
                di = (di + 1) % 4;
                r += dr[di];
                c += dc[di];
            }
        }

        res
    }

    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut res = Vec::new();

        let (mut rt, mut rb) = (0, matrix.len() - 1);    // Top row and Bottom row
        let (mut cl, mut cr) = (0, matrix[0].len() - 1); // Left column and Right column

        while rt <= rb && cl <= cr {
            (cl..=cr).for_each(|c| res.push(matrix[rt][c]));
            (rt+1..=rb).for_each(|r| res.push(matrix[r][cr]));

            if rt < rb && cl < cr {
                (cl..cr).rev().for_each(|c| res.push(matrix[rb][c]));
                (rt+1..rb).rev().for_each(|r| res.push(matrix[r][cl]));
            }

            rt += 1;
            rb -= 1;
            cl += 1;
            cr -= 1;
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
            vec![1, 2, 3, 6, 9, 8, 7, 4, 5],
            Solution::spiral_order(
                vec![
                    vec![1, 2, 3],
                    vec![4, 5, 6],
                    vec![7, 8, 9],
                ],
            ),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7],
            Solution::spiral_order(
                vec![
                    vec![1, 2,  3,  4],
                    vec![5, 6,  7,  8],
                    vec![9, 10, 11, 12],
                ],
            ),
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            vec![1, 2, 3, 4, 8, 12, 16, 15, 14, 13, 9, 5, 6, 7, 11, 10],
            Solution::spiral_order(
                vec![
                    vec![1,  2,  3,  4],
                    vec![5,  6,  7,  8],
                    vec![9,  10, 11, 12],
                    vec![13, 14, 15, 16],
                ]
            ),
        );
    }
}
