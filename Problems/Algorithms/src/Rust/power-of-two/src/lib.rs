// https://leetcode.com/problems/power-of-two/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n -1)) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            true,
            Solution::is_power_of_two(1),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            true,
            Solution::is_power_of_two(16),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            false,
            Solution::is_power_of_two(218),
        );
    }
}
