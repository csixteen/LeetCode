// https://leetcode.com/problems/valid-perfect-square/

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let num: i64 = num as i64;
        let mut lo = 1;
        let mut hi = (num / 2) as i64;

        while lo <= hi {
            let mid = lo + ((hi - lo) / 2) as i64;
            
            match (mid * mid).cmp(&num) {
                Ordering::Less => lo = mid + 1,
                Ordering::Greater => hi = mid - 1,
                Ordering::Equal => { return true; }
            }
        }

        false || num == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        num: i32,
        expected: bool,
    }

    #[test]
    fn test_is_perfect_square() {
        let test_cases = [
            TestCase {
                num: 808201,
                expected: true,
            },
            TestCase {
                num: 16,
                expected: true,
            },
            TestCase {
                num: 14,
                expected: false,
            },
            TestCase {
                num: 2,
                expected: false,
            },
            TestCase {
                num: 1,
                expected: true,
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::is_perfect_square(case.num),
            );
        }
    }
}
