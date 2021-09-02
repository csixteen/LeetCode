// https://leetcode.com/problems/longest-common-prefix/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        fn longest(a: &String, b: &String) -> String {
            a
                .chars()
                .zip(b.chars())
                .take_while(|(x, y)| x == y)
                .map(|(x, _)| x)
                .collect()
        }

        if strs.is_empty() { return String::from("") }

        strs.iter().skip(1).fold(strs[0].clone(), |acc, x| {
            longest(&acc, x)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            String::from("fl"),
            Solution::longest_common_prefix(
                vec![
                    String::from("flower"),
                    String::from("flow"),
                    String::from("flight"),
                ],
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            String::from(""),
            Solution::longest_common_prefix(
                vec![
                    String::from("dog"),
                    String::from("racecar"),
                    String::from("car"),
                ],
            ),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            String::from(""),
            Solution::longest_common_prefix(vec![]),
        );
    }
}
