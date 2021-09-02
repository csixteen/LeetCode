// https://leetcode.com/problems/number-of-good-pairs/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        nums.iter().fold(HashMap::new(), |mut acc, k| {
            *acc.entry(k).or_insert(0) += 1;
            acc
        }).values().fold(0, |acc, n| acc + n * (n-1) / 2)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        assert_eq!(4, Solution::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]));
    }

    #[test]
    fn test_example2() {
        assert_eq!(6, Solution::num_identical_pairs(vec![1, 1, 1, 1]));
    }

    #[test]
    fn test_example3() {
        assert_eq!(0, Solution::num_identical_pairs(vec![1, 2, 3]));
    }
}
