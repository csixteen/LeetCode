// https://leetcode.com/problems/top-k-frequent-elements/

#![allow(dead_code)]

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let groups: HashMap<i32, i32> = nums.iter().fold(HashMap::new(), |mut acc, c| {
            acc.entry(*c).and_modify(|e| { *e += 1 }).or_insert(1);
            acc
        });

        let mut freqs = groups
            .iter()
            .fold(Vec::<(i32,i32)>::new(), |mut acc, (k,v)| {
                acc.push((*k, *v));
                acc
            });
        freqs.sort_by(|a, b| (*a).1.cmp(&b.1).reverse());
        freqs.iter().map(|x| x.0).take(k as usize).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_frequent() {
        assert_eq!(
            Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2),
            vec![1, 2],
        );
        assert_eq!(
            Solution::top_k_frequent(vec![1], 1),
            vec![1],
        );
    }
}
