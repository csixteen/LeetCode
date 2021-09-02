// https://leetcode.com/problems/running-sum-of-1d-array/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        (0..nums.len()).fold(vec![0], |mut acc, i| {
            acc.push(nums[i] + acc[i]);
            acc
        })[1..].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::running_sum(vec![1, 2, 3, 4]),
            vec![1, 3, 6, 10],
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::running_sum(vec![1, 1, 1, 1, 1]),
            vec![1, 2, 3, 4, 5],
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            Solution::running_sum(vec![3, 1, 2, 10, 1]),
            vec![3, 4, 6, 16, 17],
        );
    }

    #[test]
    fn test_example4() {
        assert_eq!(
            Solution::running_sum(vec![]),
            Vec::new(),
        );
    }
}
