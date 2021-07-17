// https://leetcode.com/problems/flipping-an-image/

struct Solution;

impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        image
            .iter()
            .map(|row| row.iter().rev().map(|d| !d & 1).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            vec![vec![1, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            Solution::flip_and_invert_image(
                vec![vec![1, 1, 0], vec![1, 0, 1], vec![0, 0, 0]],
            ),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            vec![vec![1, 1, 0, 0], vec![0, 1, 1, 0], vec![0, 0, 0, 1], vec![1, 0, 1, 0]],
            Solution::flip_and_invert_image(
                vec![vec![1, 1, 0, 0], vec![1, 0, 0, 1], vec![0, 1, 1, 1], vec![1, 0, 1, 0]],
            ),
        );
    }
}
