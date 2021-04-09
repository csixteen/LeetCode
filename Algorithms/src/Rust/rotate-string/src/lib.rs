// https://leetcode.com/problems/rotate-string/

struct Solution;

impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        let (len_a, len_b) = (a.len(), b.len());
        let mut a = a;
        a.push_str(&a.clone());
        (len_a == len_b) && a.contains(&b)
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
