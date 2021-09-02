// https://leetcode.com/problems/intersection-of-two-arrays-ii/

#![allow(dead_code)]
#![allow(non_snake_case)]

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut count1: HashMap<i32, usize> = nums1
            .iter()
            .fold(HashMap::new(), |mut acc, n| {
                acc
                    .entry(*n)
                    .and_modify(|e| {*e += 1 })
                    .or_insert(1);
                acc
            });

        let mut res: Vec<i32> = Vec::new();
        nums2.iter().for_each(|n| {
            match count1.contains_key(n) {
                true if count1.get(n).unwrap() > &0 => {
                    res.push(*n);
                    count1.entry(*n).and_modify(|e| { *e -= 1 });
                },
                _ => (),
            }
        });

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![2, 2],
            Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            vec![9, 4],
            Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
        );
    }
}
