// https://leetcode.com/problems/binary-number-with-alternating-bits/

struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let x = (n ^ (n >> 1)) + 1;
        x & (x - 1) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert!(Solution::has_alternating_bits(5));
    }

    #[test]
    fn example2() {
        assert!(!Solution::has_alternating_bits(7));
    }

    #[test]
    fn example3() {
        assert!(!Solution::has_alternating_bits(11));
    }

    #[test]
    fn exampl4() {
        assert!(Solution::has_alternating_bits(10));
    }

    #[test]
    fn example5() {
        assert!(!Solution::has_alternating_bits(3));
    }

    #[test]
    fn example6() {
        assert!(Solution::has_alternating_bits(0x5555_5555));
    }
}
