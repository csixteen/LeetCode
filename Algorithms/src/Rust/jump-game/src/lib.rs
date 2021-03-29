#![allow(dead_code)]

use std::cmp;
use std::convert::TryInto;

struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut span: usize = 0;
        let mut i: usize = 0;

        while i < nums.len() - 1 {
            if i > span {
                return false;
            }

            span = cmp::max(span, (i as i32 + nums[i]).try_into().unwrap());
            i += 1;
        }

        nums.len() - 1 <= span
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        nums: Vec<i32>,
        expected: bool,
    }

    #[test]
    fn test_can_jump() {
        let test_cases = [
            TestCase {
                nums: vec![0],
                expected: true,
            },
            TestCase {
                nums: vec![0, 1],
                expected: false,
            },
            TestCase {
                nums: vec![2, 5, 0, 0],
                expected: true,
            },
            TestCase {
                nums: vec![2, 0, 0],
                expected: true,
            },
            TestCase {
                nums: vec![2, 3, 1, 1, 4],
                expected: true,
            },
            TestCase {
                nums: vec![3, 2, 1, 0, 4],
                expected: false,
            },
            TestCase {
                nums: vec![5, 0, 0, 0, 0, 3, 1, 0, 4],
                expected: true,
            },
            TestCase {
                nums: vec![4, 3, 0, 0, 0, 3, 1, 0, 4],
                expected: false,
            }
        ];

        for case in test_cases.iter() {
            assert_eq!(case.expected, Solution::can_jump(case.nums.to_vec()));
        }
    }
}
