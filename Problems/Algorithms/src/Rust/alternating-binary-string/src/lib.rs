// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/

use std::str::Chars;

struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        fn go(mut iter: Chars, next: char, acc: i32) -> i32 {
            match iter.next() {
                None => acc,
                Some(c) => {
                    let new_acc = acc + if c == next { 0 } else { 1 };
                    let new_next = if next == '0' { '1' } else { '0' };
                    go(iter, new_next, new_acc)
                }
            }
        }

        go(s.chars(), '0', 0).min(go(s.chars(), '1', 0))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        assert_eq!(1, Solution::min_operations("0100".to_string()));
    }

    #[test]
    fn test_example2() {
        assert_eq!(0, Solution::min_operations("10".to_string()));
    }

    #[test]
    fn test_example3() {
        assert_eq!(2, Solution::min_operations("1111".to_string()));
    }
}
