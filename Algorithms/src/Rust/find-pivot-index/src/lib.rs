// https://leetcode.com/problems/find-pivot-index/

struct Solution;

impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        if nums.is_empty() { return -1 }

        let mut left = 0;
        let mut right = nums.iter().sum::<i32>();

        for i in 0..nums.len() {
            right -= nums[i];
            if left == right { return i as i32 }
            left += nums[i];
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn test_example2() {
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn test_example3() {
        assert_eq!(Solution::pivot_index(vec![1]), 0);
    }
}
