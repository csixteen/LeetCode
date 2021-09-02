// https://leetcode.com/problems/two-sum/

#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indices: HashMap<i32, i32> = HashMap::new();

        for (index, i) in nums.iter().enumerate() {
            match indices.get(&(target - i)) {
                None => { indices.insert(*i, index as i32); },
                Some(j) => return vec![*j, index as i32],
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![0, 1],
            Solution::two_sum(vec![2, 7, 11, 15], 9),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            vec![0, 3],
            Solution::two_sum(vec![2, 7, 11, 15], 17),
        );
    }
}
