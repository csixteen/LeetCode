// https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/

struct Solution;

impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let min_prefix_sum = nums.iter().fold((0, i32::MAX), |(s, p), i| {
            (s + i, p.min(s+i))
        });

        1.max(1 - min_prefix_sum.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            5,
            Solution::min_start_value(vec![-3,2,-3,4,2]),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            1,
            Solution::min_start_value(vec![1, 2]),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            5,
            Solution::min_start_value(vec![1, -2, -3]),
        );
    }
}
