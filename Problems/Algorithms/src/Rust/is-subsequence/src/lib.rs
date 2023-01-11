// https://leetcode.com/problems/is-subsequence/

#![allow(dead_code)]

struct Solution; 

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        fn _is_subsequence(s_bytes: &[u8], t_bytes: &[u8]) -> bool {
            if s_bytes.len() == 0 { return true; }
            if t_bytes.len() == 0 { return false; }

            if s_bytes[0] == t_bytes[0] {
                return _is_subsequence(&s_bytes[1..], &t_bytes[1..]);
            } else {
                return _is_subsequence(s_bytes, &t_bytes[1..]);
            }
        }

        _is_subsequence(s.as_bytes(), t.as_bytes())
    }

    pub fn is_subsequence2(s: String, t: String) -> bool {
        let mut ti = t.chars();
        s.chars().all(|sc| ti.any(|tc| tc == sc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            true,
            Solution::is_subsequence(
                String::from("abc"), String::from("ahbgdc"),
            ),
        );
        assert_eq!(
            true,
            Solution::is_subsequence2(
                String::from("abc"), String::from("ahbgdc"),
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            false,
            Solution::is_subsequence(
                String::from("axc"), String::from("ahbgdc"),
            ),
        );
        assert_eq!(
            false,
            Solution::is_subsequence2(
                String::from("axc"), String::from("ahbgdc"),
            ),
        );
    }
}
