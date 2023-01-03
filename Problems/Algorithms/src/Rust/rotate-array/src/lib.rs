#![allow(dead_code)]

use std::mem;

struct Solution;

impl Solution {
    pub fn rotate_linear_space(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        if len < 2 || k == 0 { return () }

        let mut tmp: Vec<i32> = Vec::with_capacity(len);

        for i in len-k..len {
            tmp.push(nums[i]);
        }

        for i in 0..len-k {
            tmp.push(nums[i]);
        }

        let _ = mem::replace(nums, tmp);
    }

    pub fn rotate_constant_space(nums: &mut Vec<i32>, k: i32) {
        let len = nums.len();
        let k = k as usize % len;
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate_constant_space(&mut v, 3);
        assert_eq!(vec![5, 6, 7, 1, 2, 3, 4], v);
    }

    #[test]
    fn test_example2() {
        let mut v = vec![-1, -100, 3, 99];
        Solution::rotate_constant_space(&mut v, 2);
        assert_eq!(vec![3, 99, -1, -100], v);
    }

    #[test]
    fn test_example3() {
        let mut v = vec![-1, -100, 3, 99];
        Solution::rotate_constant_space(&mut v, 3);
        assert_eq!(vec![-100, 3, 99, -1], v);
    }

    #[test]
    fn test_example4() {
        let mut v = vec![-1, -100, 3, 99, 5];
        Solution::rotate_constant_space(&mut v, 2);
        assert_eq!(vec![99, 5, -1, -100, 3], v);
    }

    #[test]
    fn test_example5() {
        let mut v = vec![1];
        Solution::rotate_constant_space(&mut v, 1);
        assert_eq!(vec![1], v);
    }

    #[test]
    fn test_example6() {
        let mut v = vec![1, 2];
        Solution::rotate_constant_space(&mut v, 2);
        assert_eq!(vec![1, 2], v);
    }
}
