// https://leetcode.com/problems/search-insert-position/

use std::cmp::Ordering;

struct Solution {}

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo: i32 = 0;
        let mut hi: i32 = nums.len() as i32 - 1;

        while lo <= hi {
            let mid: i32 = lo + (hi - lo) / 2;

            match target.cmp(&nums[mid as usize]) {
                Ordering::Equal => { return mid; },
                Ordering::Less => { hi = mid - 1; },
                Ordering::Greater => { lo = mid + 1; },
            }
        }

        lo
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            2,
            Solution::search_insert(vec![1, 3, 5, 6], 5),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            1,
            Solution::search_insert(vec![1, 3, 5, 6], 2),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            4,
            Solution::search_insert(vec![1, 3, 5, 6], 7),
        );
    }

    #[test]
    fn test_example4() {
        assert_eq!(
            0,
            Solution::search_insert(vec![1, 3, 5, 6], 0),
        );
    }
}
