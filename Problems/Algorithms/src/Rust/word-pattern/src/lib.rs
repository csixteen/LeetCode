// https://leetcode.com/problems/word-pattern/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let mut pattern_word: HashMap<char, &str> = HashMap::new();
        let mut word_pattern: HashMap<&str, char> = HashMap::new();

        let words: Vec<&str> = s.split(' ').collect();

        for (c, w) in pattern.chars().zip(words.iter()) {
            if !pattern_word.contains_key(&c) &&
                !word_pattern.contains_key(w) {
                pattern_word.insert(c, w);
                word_pattern.insert(w, c);
                continue;
            }

            match (pattern_word.get(&c), word_pattern.get(w)) {
                (Some(w1), Some(c1)) if c == *c1 && w == w1 => (),
                _ => { return false },
            }
        }

        pattern.len() == words.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert!(Solution::word_pattern(
            "abba".to_string(),
            "dog cat cat dog".to_string(),
        ));
    }

    #[test]
    fn example2() {
        assert!(!Solution::word_pattern(
            "abba".to_string(),
            "dog cat cat fish".to_string(),
        ));
    }

    #[test]
    fn example3() {
        assert!(!Solution::word_pattern(
            "aaaa".to_string(),
            "dog cat cat dog".to_string(),
        ));
    }

    #[test]
    fn example4() {
        assert!(!Solution::word_pattern(
            "abba".to_string(),
            "dog dog dog dog".to_string(),
        ));
    }
}
