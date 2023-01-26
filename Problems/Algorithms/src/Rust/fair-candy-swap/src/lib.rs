// https://leetcode.com/problems/fair-candy-swap/

#![allow(dead_code)]

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn fair_candy_swap2(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let sum_a = alice_sizes.iter().sum::<i32>();
        let sum_b = bob_sizes.iter().sum::<i32>();
        let set_a = alice_sizes.iter().fold(HashSet::new(), |mut acc, x| {
            acc.insert(x);
            acc
        });
        bob_sizes.iter().find_map(|y| {
            let x = (sum_a - sum_b) / 2 + y;
            if set_a.contains(&x) { Some(vec![x, *y]) } else { None }
        }).unwrap()
    }

    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let mut set_a = HashSet::new();
        let mut sum_a = 0;
        for a in alice_sizes.iter() {
            set_a.insert(a);
            sum_a += a;
        }

        let sum_b = bob_sizes.iter().sum::<i32>();
        let xxx = (sum_a - sum_b) / 2;
        bob_sizes.iter().find_map(|y| {
            let x = xxx + y;
            if set_a.contains(&x) { Some(vec![x, *y]) } else { None }
        }).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            vec![1, 2],
            Solution::fair_candy_swap(vec![1, 1], vec![2, 2]),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            vec![1, 2],
            Solution::fair_candy_swap(vec![1, 2], vec![2, 3]),
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            vec![2, 3],
            Solution::fair_candy_swap(vec![2], vec![1, 3]),
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            vec![5, 4],
            Solution::fair_candy_swap(vec![1, 2, 5], vec![2, 4]),
        );
    }
}
