// https://leetcode.com/problems/hamming-distance/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(2, Solution::hamming_distance(1, 4));
    }
}
