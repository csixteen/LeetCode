// https://leetcode.com/problems/rectangle-overlap/

#![allow(dead_code)]

struct Solution;

impl Solution {
    fn zero_area(rec: &Vec<i32>) -> bool {
        rec[0] == rec[2] || rec[1] == rec[3]
    }

    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        if rec1[2] <= rec2[0] ||
           rec2[2] <= rec1[0] ||
           rec1[3] <= rec2[1] ||
           rec2[3] <= rec1[1] ||
           Self::zero_area(&rec1) ||
           Self::zero_area(&rec2) {
            return false
        }

        true
    }

    pub fn is_rectangle_overlap2(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        // Vertical intersection
        let vi = rec1[1].max(rec2[1]) < rec1[3].min(rec2[3]);

        // Horizontal intersection
        let hi = rec1[0].max(rec2[0]) < rec1[2].min(rec2[2]);

        // The rectangles overlap if there are both horizontal and vertical
        // intersections.
        vi && hi
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert!(Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]));
        assert!(Solution::is_rectangle_overlap2(vec![0, 0, 2, 2], vec![1, 1, 3, 3]));
    }

    #[test]
    fn test_example2() {
        assert!(!Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1]));
        assert!(!Solution::is_rectangle_overlap2(vec![0, 0, 1, 1], vec![1, 0, 2, 1]));
    }

    #[test]
    fn test_example3() {
        assert!(!Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![2, 2, 3, 3]));
        assert!(!Solution::is_rectangle_overlap2(vec![0, 0, 1, 1], vec![2, 2, 3, 3]));
    }

    #[test]
    fn test_example4() {
        assert!(!Solution::is_rectangle_overlap(vec![-1, 0, 1, 1], vec![0, -1, 0, 1]));
        assert!(!Solution::is_rectangle_overlap2(vec![-1, 0, 1, 1], vec![0, -1, 0, 1]));
    }
}
