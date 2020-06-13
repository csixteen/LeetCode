// https://leetcode.com/problems/largest-divisible-subset/

#![allow(dead_code)]

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        &nums.sort_unstable();

        let mut subsets: HashMap<i32, Vec<i32>> = HashMap::new();
        subsets.insert(-1, Vec::new());

        for n in nums.iter() {
            let mut s = subsets
                .iter()
                .filter(|(k, _)| *n % **k == 0)
                .map(|(_, v)| v)
                .max_by_key(|v| v.len())
                .unwrap()
                .to_vec();

            s.push(*n);

            subsets.insert(*n, s);
        }

        subsets.values().max_by_key(|v| v.len()).unwrap().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let ret = Solution::largest_divisible_subset(vec![1, 2, 3]);
        assert!(ret == vec![1, 2] || ret == vec![1, 3]);
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            vec![1, 2, 4, 8],
            Solution::largest_divisible_subset(vec![1, 2, 4, 8]),
        );
    }

    #[test]
    fn test_example3() {
        let ret = Solution::largest_divisible_subset(vec![1, 2, 3, 4, 15]);
        assert!(ret == vec![1, 2, 4] || ret == vec![1, 3, 15]);
    }

    #[test]
    fn test_example4() {
        assert_eq!(
            vec![4, 8, 240],
            Solution::largest_divisible_subset(vec![4, 8, 10, 240]),
        );
    }

    #[test]
    fn test_example5() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::largest_divisible_subset(Vec::new()),
        );
    }

    #[test]
    fn test_example6() {
        assert_eq!(
            vec![4, 8, 16],
            Solution::largest_divisible_subset(vec![3, 4, 16, 8]),
        );
    }
}
