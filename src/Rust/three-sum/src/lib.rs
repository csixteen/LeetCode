// https://leetcode.com/problems/3sum/

#![allow(dead_code)]

use std::cmp::Ordering;

struct Solution;

impl Solution {
    fn two_sum(nums: &Vec<i32>, i: usize) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let (mut lo, mut hi) = (i + 1, nums.len() - 1);

        while lo < hi {
            match (nums[i] + nums[lo] + nums[hi]).cmp(&0) {
                Ordering::Less => { lo += 1; },
                Ordering::Greater => { hi -= 1; }
                Ordering::Equal => {
                    res.push(vec![nums[i], nums[lo], nums[hi]]);
                    lo += 1;
                    hi -= 1;

                    // To avoid duplicate results
                    while lo < hi && nums[lo] == nums[lo-1] {
                        lo += 1;
                    }
                },
            }
        }

        res
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut nums = nums;

        &nums.sort_unstable();

        for i in 0..nums.len() {
            if nums[i] > 0 { break ; }

            // To avoid duplicate results
            if i == 0 || nums[i] != nums[i-1] {
                res.append(&mut Self::two_sum(&nums, i));
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![
                vec![-1, -1, 2],
                vec![-1, 0, 1],
            ],
            Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
            // -4 -1 -1 0 1 2
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            Solution::three_sum(vec![]),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            Solution::three_sum(vec![0]),
        );
    }
}
