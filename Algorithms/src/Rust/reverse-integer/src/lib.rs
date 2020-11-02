// https://leetcode.com/problems/reverse-integer/

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        match x.is_positive() {
            true => Self::rev_int(x).unwrap_or(0),
            false => Self::rev_int(-1 * x).unwrap_or(0) * -1,
        }
    }

    fn rev_int(x: i32) -> Result<i32, std::num::ParseIntError> {
        x.to_string().chars().rev().collect::<String>().parse::<i32>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(321, Solution::reverse(123));
    }

    #[test]
    fn test_example2() {
        assert_eq!(-321, Solution::reverse(-123));
    }

    #[test]
    fn test_example3() {
        assert_eq!(0, Solution::reverse(1111111118));
    }
}
