// https://leetcode.com/problems/missing-ranges/

#![allow(dead_code)]

use std::fmt;

struct Solution;

impl Solution {
    fn range_str(lo: i64, hi: i64) -> String {
        if lo == hi {
            lo.to_string()
        } else {
            fmt::format(
                format_args!(
                    "{}->{}",
                    lo.to_string(),
                    hi.to_string(),
                )
            )
        }
    }

    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
        let (lower, upper) = (lower as i64, upper as i64);
        let mut res = Vec::new();
        let mut previous = lower - 1;

        for i in nums {
            let i = i as i64;

            if previous + 1 < i {
                res.push(Self::range_str(previous + 1, i - 1));
            }

            previous = i;
        }

        if previous != upper {
            res.push(Self::range_str(previous + 1, upper));
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
            vec!["2", "4->49", "51->74", "76->99"],
            Solution::find_missing_ranges(
                vec![0, 1, 3, 50, 75],
                0,
                99,
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            vec!["1"],
            Solution::find_missing_ranges(vec![], 1, 1),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            vec!["-2"],
            Solution::find_missing_ranges(vec![-1], -2, -1),
        );
    }

    #[test]
    fn test_example4() {
        assert_eq!(
            vec!["-5->-1", "2", "4->49", "51->74", "76->99"],
            Solution::find_missing_ranges(
                vec![0, 1, 3, 50, 75],
                -5,
                99,
            ),
        );
    }
}
