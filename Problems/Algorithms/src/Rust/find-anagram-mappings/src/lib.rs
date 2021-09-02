// https://leetcode.com/problems/find-anagram-mappings/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn anagram_mappings(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let map2: HashMap<i32, usize> = nums2
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, n)| { acc.insert(*n, i); acc });

        nums1.iter().map(|n| *map2.get(n).unwrap() as i32).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            vec![1, 4, 3, 2, 0],
            Solution::anagram_mappings(vec![12, 28, 46, 32, 50], vec![50, 12, 32, 46, 28]),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            vec![0, 1],
            Solution::anagram_mappings(vec![84, 46], vec![84, 46]),
        );
    }
}
