// https://leetcode.com/problems/merge-intervals/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        &intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        intervals.iter().fold(Vec::new(), |mut acc, i| {
            if !acc.is_empty() && i[0] <= acc[acc.len()-1][1] {
                let last = acc.pop().unwrap();
                acc.push(vec![last[0], last[1].max(i[1])]);
                acc
            }
            else { acc.push(i.clone()); acc }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_example1() {
        assert_eq!(
            Solution::merge(vec![vec![1, 3], vec![2, 6],vec![8, 10],vec![15, 18]]),
            vec![vec![1, 6], vec![8, 10], vec![15, 18]],
        );
    }

    #[test]
    fn test_merge_example2() {
        assert_eq!(
            Solution::merge(vec![vec![1, 4], vec![4, 5]]),
            vec![vec![1, 5]],
        );
    }
}
