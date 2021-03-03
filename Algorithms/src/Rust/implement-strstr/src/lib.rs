// https://leetcode.com/problems/implement-strstr/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let (hlen, nlen) = (haystack.len(), needle.len());

        if nlen == 0 { return 0; }
        else if hlen < nlen { return -1; }

        for i in 0..=(hlen - nlen) {
            if &haystack[i..i+nlen] == needle {
                return i as i32
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            2,
            Solution::str_str(String::from("hello"), String::from("ll")),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            -1,
            Solution::str_str(String::from("aaaaa"), String::from("bba")),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            0,
            Solution::str_str(String::from("a"), String::from("a")),
        );
    }
    
    #[test]
    fn test_example4() {
        assert_eq!(
            -1,
            Solution::str_str(String::from(""), String::from("a")),
        );
    }
}
