// https://leetcode.com/problems/sort-characters-by-frequency/

#![allow(dead_code)]

use std::collections::HashMap;
use std::iter;

struct Solution;

impl Solution {
    fn char_counter(s: &String) -> HashMap<char, usize> {
        let mut counter = HashMap::new();

        s.chars().for_each(|c| {
            counter
                .entry(c)
                .and_modify(|e| { *e += 1 })
                .or_insert(1);
        });

        counter
    }

    pub fn frequency_sort(s: String) -> String {
        if s.is_empty() { return String::from("") }

        let char_freq = Self::char_counter(&s);
        let (min, max) = (
            char_freq.values().min().unwrap(),
            char_freq.values().max().unwrap(),
        );
        let group = char_freq
            .iter()
            .fold(vec![Vec::<char>::new(); max - min + 1], |mut acc, (k, v)| {
                acc[*v - min].push(*k);
                acc
            });

        group
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, chars)| !chars.is_empty())
            .map(|(i, chars)| {
                chars
                    .iter()
                    .map(|c| iter::repeat(c).take(i+min).collect::<String>())
                    .collect::<Vec<String>>()
                    .concat()
            })
            .collect::<Vec<String>>()
            .concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    #[test]
    fn test_example1() {
        assert!(
            [String::from("eetr"), String::from("eert")]
                .iter()
                .cloned()
                .collect::<HashSet<String>>()
                .contains(
                    &Solution::frequency_sort(String::from("tree"))
                )
        );
    }

    #[test]
    fn test_example2() {
        assert!(
            [String::from("cccaaa"), String::from("aaaccc")]
                .iter()
                .cloned()
                .collect::<HashSet<String>>()
                .contains(
                    &Solution::frequency_sort(String::from("cccaaa"))
                )
        );
    }

    #[test]
    fn test_example3() {
        assert!(
            [String::from("bbaA"), String::from("bbAa")]
                .iter()
                .cloned()
                .collect::<HashSet<String>>()
                .contains(
                    &Solution::frequency_sort(String::from("Aabb"))
                )
        );
    }
}
