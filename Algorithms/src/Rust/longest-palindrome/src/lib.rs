// https://leetcode.com/problems/longest-palindrome/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let count = s.chars().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|e| { *e += 1 }).or_insert(1);
            acc
        });

        let t = count.values().fold(0, |acc, v| acc + ((*v / 2) * 2));
        t + ((t < s.len() as i32) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_longest_palindrome() {
        assert_eq!(7, Solution::longest_palindrome("abccccdd".to_string()));
        assert_eq!(1, Solution::longest_palindrome("a".to_string()));
        assert_eq!(2, Solution::longest_palindrome("bb".to_string()));
    }
}
