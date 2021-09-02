// https://leetcode.com/problems/subsets/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn go(xs: &Vec<i32>, i: usize, acc: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            if i >= xs.len() { acc.to_vec() }
            else {
                acc.append(&mut acc.clone().iter().map(|x| {
                    let mut y = x.clone();
                    y.push(xs[i]);
                    y
                }).collect());
                go(xs, i+1, acc)
            }
        }

        go(&nums, 0, &mut vec![vec![]])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        assert_eq!(
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ],
            Solution::subsets(vec![1, 2, 3]),
        );
    }
}
