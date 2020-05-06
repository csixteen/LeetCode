// https://leetcode.com/problems/majority-element/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let number_freq = nums
            .iter()
            .fold(HashMap::new(), |mut acc, n| {
                acc
                    .entry(n)
                    .and_modify(|e| { *e += 1 })
                    .or_insert(1);
                acc
            });

        for (number, freq) in number_freq.iter() {
            if freq > &((nums.len() / 2) as i32) {
                return **number;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        nums: Vec<i32>,
        expected: i32,
    }

    #[test]
    fn test_majority_element() {
        let test_cases = [
            TestCase {
                nums: vec![3, 2, 3],
                expected: 3,
            },
            TestCase {
                nums: vec![2, 2, 1, 1, 1, 2, 2],
                expected: 2,
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::majority_element(case.nums.clone()),
            );
            assert_eq!(
                case.expected,
                Solution::majority_element2(case.nums.clone()),
            );
        }
    }
}
