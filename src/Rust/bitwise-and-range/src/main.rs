// https://leetcode.com/problems/bitwise-and-of-numbers-range/

struct Solution {}

impl Solution {
    // Brute-force solution (accepted)
    pub fn range_bitwise_and(m: i32, n: i32) -> i32 {
        (m..=n).fold(i32::max_value(), |acc, x| acc & x)
    }
}
