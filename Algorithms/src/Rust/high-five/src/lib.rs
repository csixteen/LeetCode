// https://leetcode.com/problems/high-five/

use std::collections::HashMap;


struct Solution;

impl Solution {
    pub fn high_five(items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut grades: Vec<Vec<i32>> =
            items
                .iter()
                .fold(HashMap::new(), |mut acc, i| {
                    let g = acc.entry(i[0]).or_insert(Vec::new()).push(i[1]);
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::high_five(
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
                ]
            ),
            vec![vec![1,87], vec![2,88]],
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::high_five(
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
                ]
            ),
            vec![vec![1,100], vec![7,100]],
        );
    }
}
