// https://leetcode.com/problems/reverse-words-in-a-string-iii/

struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        s
            .split(" ")
            .map(|w| w.chars().rev().collect())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_words() {
        assert_eq!(
            String::from("s'teL ekat edoCteeL tsetnoc"),
            Solution::reverse_words(String::from("Let's take LeetCode contest"))
        );

        assert_eq!(
            String::from("doG gniD"),
            Solution::reverse_words(String::from("God Ding"))
        );
    }
}
