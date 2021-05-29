// https://leetcode.com/problems/count-substrings-with-only-one-distinct-letter/

use std::str::Chars;

struct Solution;

impl Solution {
    pub fn count_letters(s: String) -> i32 {
        fn go(mut iter: Chars, prev: Option<char>, curr: i32, acc: i32) -> i32 {
            match (prev, iter.next()) {
                (_, None) => acc + curr*(curr + 1)/2,
                (p, Some(c)) =>
                    if p.is_none() || p.unwrap() == c {
                        go(iter, Some(c), curr+1, acc)
                    } else {
                        go(iter, Some(c), 1, acc + curr*(curr + 1)/2)
                    },
            }
        }

        go(s.chars(), None, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        assert_eq!(8, Solution::count_letters("aaaba".to_string()));
    }

    #[test]
    fn test_example2() {
        assert_eq!(55, Solution::count_letters("aaaaaaaaaa".to_string()));
    }
}
