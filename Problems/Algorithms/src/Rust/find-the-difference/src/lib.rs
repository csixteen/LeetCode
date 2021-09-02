// https://leetcode.com/c0x10/

struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        s.bytes().chain(t.bytes()).fold(0_u8, |acc, c| acc ^ c) as char
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            'e',
            Solution::find_the_difference("abcd".to_string(), "abcde".to_string()),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            'y',
            Solution::find_the_difference("".to_string(), "y".to_string()),
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            'a',
            Solution::find_the_difference("a".to_string(), "aa".to_string()),
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            'a',
            Solution::find_the_difference("ae".to_string(), "aea".to_string()),
        );
    }
}
