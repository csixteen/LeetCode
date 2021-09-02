// https://leetcode.com/problems/maximum-population-year/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn maximum_population(logs: Vec<Vec<i32>>) -> i32 {
        let mut count: Vec<(i32, i32)> = logs
            .iter()
            .fold(HashMap::new(), |mut acc, pair| {
                (pair[0]..pair[1]).for_each(|i| {
                    acc.entry(i).and_modify(|e| { *e += 1 }).or_insert(1);
                });
                acc
            })
            .into_iter()
            .collect();

        count.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        count[0].0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            1993,
            Solution::maximum_population(vec![vec![1993, 1999], vec![2000, 2010]]),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            1960,
            Solution::maximum_population(
                vec![
                    vec![1950, 1961],
                    vec![1960, 1971],
                    vec![1970, 1981]
                ],
            ),
        );
    }
}
