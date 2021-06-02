// https://leetcode.com/problems/repeated-substring-pattern/

struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        (s.clone() + &s)[1..s.len()*2-1].contains(&s)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        assert!(Solution::repeated_substring_pattern("abab".to_string()));
    }

    #[test]
    fn test_example2() {
        assert!(!Solution::repeated_substring_pattern("aba".to_string()));
    }

    #[test]
    fn test_example3() {
        assert!(Solution::repeated_substring_pattern("abcabcabcabc".to_string()));
    }
}
