// https://leetcode.com/problems/longest-substring-without-repeating-characters/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut indices: HashMap<char, usize> = HashMap::new();
        let (mut start, mut end, mut longest) = (0, 0, 0);

        for c in s.chars() {
            if indices.contains_key(&c) {
                start = start.max(indices[&c] + 1);
            }

            indices.insert(c, end);
            end += 1;
            longest = longest.max(end - start);
        }

        longest as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("abcabcbb".to_string()),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            1,
            Solution::length_of_longest_substring("bbbbb".to_string()),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            3,
            Solution::length_of_longest_substring("pwwkew".to_string()),
        );
    }

    #[test]
    fn test_example4() {
        assert_eq!(
            2,
            Solution::length_of_longest_substring("abba".to_string()),
        );
    }

    #[test]
    fn test_example5() {
        assert_eq!(
            1,
            Solution::length_of_longest_substring(" ".to_string()),
        );
    }
}
