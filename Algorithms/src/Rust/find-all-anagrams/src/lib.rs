// https://leetcode.com/problems/find-all-anagrams-in-a-string/

#![allow(dead_code)]

use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn char_freq(s: &String) -> HashMap<char, i32> {
        let mut counter = HashMap::new();

        s.chars().for_each(|c| {
            counter
                .entry(c)
                .and_modify(|e| { *e += 1 })
                .or_insert(1);
        });

        counter
    }

    pub fn find_anagrams(haystack: String, needle: String) -> Vec<i32> {
        let (hay_len, need_len) = (haystack.len(), needle.len());
        if need_len > hay_len { return Vec::<i32>::new();}

        let haystack: Vec<char> = haystack.chars().collect();
        let mut freq = Self::char_freq(&needle);
        let mut zeroes = freq.keys().count();
        let mut ret = Vec::new();

        // Sliding window
        let mut start = 0;
        let mut end = 0;

        while end < hay_len {
            freq.entry(haystack[end]).and_modify(|e| { *e -= 1 });

            if let Some(x) = freq.get(&haystack[end]) {
                if *x == 0 { zeroes -= 1; }
            }

            if zeroes == 0 { ret.push(start as i32); }

            end += 1;

            if end >= need_len {
                freq.entry(haystack[start]).and_modify(|e| { *e += 1 });

                if let Some(x) = freq.get(&haystack[start]) {
                    if *x == 1 { zeroes += 1; }
                }

                start += 1;
            }
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_haystack() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::find_anagrams(String::new(), String::from("abc")),
        );
    }

    #[test]
    fn test_haystack_smaller_than_needle() {
        assert_eq!(
            Vec::<i32>::new(),
            Solution::find_anagrams(
                String::from("ab"),
                String::from("abc"),
            ),
        );
    }

    #[test]
    fn test_anagram_at_the_end() {
        assert_eq!(
            vec![2],
            Solution::find_anagrams(
                String::from("xxabc"),
                String::from("cba"),
            ),
        );
    }

    #[test]
    fn test_all_as() {
        assert_eq!(
            vec![0, 1, 2, 3, 4],
            Solution::find_anagrams(
                String::from("aaaaaa"),
                String::from("aa"),
            ),
        );
    }

    #[test]
    fn test_example_1() {
        assert_eq!(
            vec![0, 6],
            Solution::find_anagrams(
                String::from("cbaebabacd"),
                String::from("abc"),
            ),
        );
    }

    #[test]
    fn test_example_2() {
        assert_eq!(
            vec![0, 1, 2],
            Solution::find_anagrams(
                String::from("abab"),
                String::from("ab"),
            ),
        );
    }
}
