// https://leetcode.com/problems/sort-array-by-increasing-frequency/

use std::collections::HashMap; 

struct Solution;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut freq: HashMap<i32, i32> = HashMap::new();
        nums.iter().for_each(|&n| {
            freq.entry(n).and_modify(|e| { *e += 1 }).or_insert(1);
        });
        nums.sort_unstable_by(|a, b| freq[a].cmp(&freq[b]).then(b.cmp(a)));
        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frequency_sort() {
        assert_eq!(
            vec![3, 1, 1, 2, 2, 2],
            Solution::frequency_sort(vec![1, 1, 2, 2, 2, 3])
        );

        assert_eq!(
            vec![1, 3, 3, 2, 2],
            Solution::frequency_sort(vec![2, 3, 1, 3, 2])
        );

        assert_eq!(
            vec![5, -1, 4, 4, -6, -6, 1, 1, 1],
            Solution::frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1])
        );
    }
}
