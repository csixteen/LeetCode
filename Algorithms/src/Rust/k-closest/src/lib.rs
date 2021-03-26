// https://leetcode.com/problems/k-closest-points-to-origin/submissions/

#![allow(dead_code)]

struct Solution; 

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points;
        &points.sort_by_key(|k| k[0].pow(2) + k[1].pow(2));
        points[..k as usize].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![vec![-2, 2]],
            Solution::k_closest(vec![vec![1, 3], vec![-2, 2]], 1),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            vec![vec![3, 3], vec![-2, 4]],
            Solution::k_closest(
                vec![vec![3, 3], vec![5, -1], vec![-2, 4]],
                2
            ),
        );
    }
}
