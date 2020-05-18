// https://leetcode.com/problems/permutation-in-string/

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

    pub fn check_inclusion(needle: String, haystack: String) -> bool {
        let (hay_len, need_len) = (haystack.len(), needle.len());
        if need_len > hay_len { return false; }

        let haystack: Vec<char> = haystack.chars().collect();
        let mut freq = Self::char_freq(&needle);
        let mut zeroes = freq.keys().count();

        // Sliding window
        let mut start = 0;
        let mut end = 0;

        while end < hay_len {
            freq.entry(haystack[end]).and_modify(|e| { *e -= 1 });

            if let Some(x) = freq.get(&haystack[end]) {
                if *x == 0 { zeroes -= 1; }
            }

            if zeroes == 0 { return true; }

            end += 1;

            if end >= need_len {
                freq.entry(haystack[start]).and_modify(|e| { *e += 1 });

                if let Some(x) = freq.get(&haystack[start]) {
                    if *x == 1 { zeroes += 1; }
                }

                start += 1;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_s1_longer_than_s2() {
        assert!(
            !Solution::check_inclusion(
                String::from("abcd"),
                String::from("abc"),
            ),
        );
    }

    #[test]
    fn test_example1() {
        assert!(
            Solution::check_inclusion(
                String::from("ab"),
                String::from("eidbaooo"),
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert!(
            !Solution::check_inclusion(
                String::from("ab"),
                String::from("eidboaoo"),
            ),
        );
    }
}
