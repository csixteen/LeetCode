#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut visited: HashMap<i32,usize> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            match visited.get(&n) {
                Some(&ii) if i - ii <= k as usize => return true,
                _ => { visited.insert(*n, i); }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 2, 3, 1], 3));
    }

    #[test]
    fn test2() {
        assert!(Solution::contains_nearby_duplicate(vec![1, 0, 1, 1], 1));
    }

    #[test]
    fn test3() {
        assert!(!Solution::contains_nearby_duplicate(vec![1, 2, 3, 1, 2, 3], 2));
    }
}
