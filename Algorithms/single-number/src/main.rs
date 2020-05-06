// https://leetcode.com/problems/single-number/

struct Solution {}

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().skip(1).fold(nums[0], |acc, x| acc^x)
    }
}
