// https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/

struct Solution;

impl Solution {
    // This one takes 4ms on LeetCode
    pub fn remove_duplicates(s: String) -> String {
        s.chars().fold(Vec::new(), |mut acc, c| {
            if !acc.is_empty() && acc[acc.len()-1] == c {
                acc.pop();
            } else {
                acc.push(c);
            }
            acc
        }).iter().collect()
    }

    // This one takes 0ms on LeetCode
    pub fn remove_duplicates2(s: String) -> String {
        let mut stack = Vec::new();

        for c in s.chars() {
            if !stack.is_empty() && stack[stack.len()-1] == c {
                stack.pop();
            } else {
                stack.push(c);
            }
        }

        stack.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            String::from("ca"),
            Solution::remove_duplicates(String::from("abbaca")),
        );
    }
}
