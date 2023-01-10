// https://leetcode.com/problems/high-five/

#![allow(dead_code)]

use std::collections::{BinaryHeap,BTreeMap,HashMap};


struct Solution;

impl Solution {
    pub fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut grades: Vec<Vec<i32>> =
            items
                .iter()
                .fold(HashMap::<i32, Vec<i32>>::new(), |mut acc, i| {
                    acc.entry(i[0]).or_default().push(i[1]);
                    acc
                })
                .iter()
                .fold(Vec::new(), |mut acc, (key, val)| {
                    let mut sorted_val = val.clone();
                    sorted_val.sort_by(|a, b| b.cmp(a));
                    acc.push(vec![*key, sorted_val.iter().take(5).sum::<i32>() / 5]);
                    acc
                });

        grades.sort_by(|a, b| a[0].cmp(&b[0]));
        grades
    }

    // Using BTreeMap and BinaryHeap
    pub fn high_five2(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        items
            .iter()
            .fold(BTreeMap::<i32, BinaryHeap<i32>>::new(), |mut acc, i| {
                acc.entry(i[0]).or_default().push(i[1]);
                acc
            })
            .into_iter()
            .map(|(id, mut grades)| {
                let mut s: i32 = 0;
                for _ in 0..5 {
                    s += grades.pop().unwrap();
                }
                vec![id, s / 5]
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let items =
            vec![
                vec![1, 91],
                vec![1, 92],
                vec![2, 93],
                vec![2, 97],
                vec![1, 60],
                vec![2, 77],
                vec![1, 65],
                vec![1, 87],
                vec![1, 100],
                vec![2, 100],
                vec![2, 76]
            ];

        assert_eq!(
            Solution::high_five(items.clone()),
            vec![vec![1,87], vec![2,88]],
        );
        assert_eq!(
            Solution::high_five2(items.clone()),
            vec![vec![1,87], vec![2,88]],
        );
    }

    #[test]
    fn test_example2() {
        let items =
            vec![
                vec![1, 100],
                vec![7, 100],
                vec![1, 100],
                vec![7, 100],
                vec![1, 100],
                vec![7, 100],
                vec![1, 100],
                vec![7, 100],
                vec![1, 100],
                vec![7, 100]
            ];

        assert_eq!(
            Solution::high_five(items.clone()),
            vec![vec![1,100], vec![7,100]],
        );
        assert_eq!(
            Solution::high_five2(items.clone()),
            vec![vec![1,100], vec![7,100]],
        );
    }
}
