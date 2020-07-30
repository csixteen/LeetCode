// https://leetcode.com/problems/number-of-1-bits/

#![allow(dead_code, non_snake_case)]

struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        (0..32).fold(0, |mut acc, i| {
            acc += (n >> i) & 1;
            acc
        }) as i32
    }

    pub fn hammingWeight2(n: u32) -> i32 {
        n.count_ones() as i32
    }

    pub fn hammingWeight3(n: u32) -> i32 {
        let mut n = n;
        let mut counter = 0;

        while n != 0 {
            counter += 1;
            n = n & (n - 1);
        }

        counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(3, Solution::hammingWeight(0x7));
        assert_eq!(3, Solution::hammingWeight2(0x7));
        assert_eq!(3, Solution::hammingWeight3(0x7));
    }

    #[test]
    fn test_example2() {
        assert_eq!(1, Solution::hammingWeight(0x100));
        assert_eq!(1, Solution::hammingWeight2(0x100));
        assert_eq!(1, Solution::hammingWeight3(0x100));
    }

    #[test]
    fn test_example3() {
        assert_eq!(31, Solution::hammingWeight(0xFFFFFFFD));
        assert_eq!(31, Solution::hammingWeight2(0xFFFFFFFD));
        assert_eq!(31, Solution::hammingWeight3(0xFFFFFFFD));
    }
}
