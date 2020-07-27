// https://leetcode.com/problems/house-robber/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut global = 0;   // The global maximum value
        let mut previous = 0; // The previous local maximum

        for n in nums.iter() {
            let temp = global;
            global = global.max(*n + previous);
            previous = temp;
        }

        global
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(4, Solution::rob(vec![1, 2, 3, 1]));
    }

    #[test]
    fn test_example2() {
        assert_eq!(12, Solution::rob(vec![2, 7, 9, 3, 1]));
    }

    #[test]
    fn test_example3() {
        assert_eq!(19, Solution::rob(vec![3, 1, 1, 16]));
    }

    #[test]
    fn test_example4() {
        assert_eq!(35, Solution::rob(vec![9, 20, 21, 10, 4, 5]));
    }
}
