// https://leetcode.com/problems/jewels-and-stones/

#![allow(dead_code)]

use std::collections::HashSet;
use std::iter::FromIterator;

struct Solution;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let jewels: HashSet<char> = HashSet::from_iter(j.chars());

        s
            .chars()
            .filter(|x| jewels.contains(x))
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        j: String,
        s: String,
        expected: i32,
    }

    #[test]
    fn test_num_jewels_in_stones() {
        let cases = [
            TestCase { j: String::from("aA"), s: String::from("aAAbbbb"), expected: 3 },
            TestCase { j: String::from("z"), s: String::from("ZZ"), expected: 0 },
        ];

        for case in cases.iter() {
            assert_eq!(
                case.expected,
                Solution::num_jewels_in_stones(case.j.clone(), case.s.clone()),
            );
        }
    }
}
