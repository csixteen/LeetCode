// https://leetcode.com/problems/rotate-string/

struct Solution;

impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        if a.len() != b.len() { return false; }

        let mut a = a;
        a.push_str(&a.clone());
        a.contains(&b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_string() {
        assert!(Solution::rotate_string(String::from("abcde"), String::from("cdeab")));
        assert!(!Solution::rotate_string(String::from("abcde"), String::from("abced")));
    }
}
