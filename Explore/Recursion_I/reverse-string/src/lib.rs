pub struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut left = 0;
        let mut right = s.len() - 1;
        let mut tmp: char;

        while left < right {
            tmp = s[left];
            s[left] = s[right];
            s[right] = tmp;

            left += 1;
            right -= 1;
        }
    }

    pub fn reverse_string2(s: &mut Vec<char>) {
        let len = s.len();
        let (left, right) = s.split_at_mut(len / 2);
        let mut left_i = 0;
        let mut right_i = right.len().saturating_sub(1);

        while left_i < left.len() {
            ::std::mem::swap(&mut left[left_i], &mut right[right_i]);
            left_i += 1;
            right_i = right_i.saturating_sub(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }

    #[test]
    fn test_case2() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }

    #[test]
    fn test_case3() {
        let mut s = vec!['a'];
        Solution::reverse_string(&mut s);
        assert_eq!(s, vec!['a']);
    }
}
