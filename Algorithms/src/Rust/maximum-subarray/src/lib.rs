// https://leetcode.com/problems/maximum-subarray/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut local = 0;
        let mut global = nums.iter().max().unwrap().clone();

        for n in nums.iter() {
            local = *n + local.max(0);
            global = global.max(local);
        }

        global
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            -1,
            Solution::max_sub_array(vec![-4, -3, -2, -1]),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            1,
            Solution::max_sub_array(vec![1, -1, 1, -1, 1, -1]),
        );
    }
}
