// https://leetcode.com/problems/reverse-string/

struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() == 0 {
            return ();
        }
        
        let mut i = 0;
        let mut j = s.len() - 1;
        
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];

        Solution::reverse_string(&mut s);

        assert_eq!(vec!['o', 'l', 'l', 'e', 'h'], s);
    }

    #[test]
    fn test_example2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];

        Solution::reverse_string(&mut s);

        assert_eq!(vec!['h', 'a', 'n', 'n', 'a', 'H'], s);
    }
}
