// https://leetcode.com/problems/increasing-triplet-subsequence/

struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let (mut first, mut second) = (i64::MAX, i64::MAX);

        for n in nums.iter() {
            if *n as i64 <= first {
                first = *n as i64;
            } else if *n as i64 <= second {
                second = *n as i64;
            } else {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert!(
            Solution::increasing_triplet(vec![1, 2, 3, 4, 5]),
        );
    }

    #[test]
    fn test_example2() {
        assert!(
            !Solution::increasing_triplet(vec![5, 4, 3, 2, 1]),
        );
    }

    #[test]
    fn test_example3() {
        assert!(
            Solution::increasing_triplet(vec![5, 2, 7, 1, 7, 3, 4]),
        );
    }

    #[test]
    fn test_example4() {
        assert!(
            !Solution::increasing_triplet(vec![1, 2]),
        );
    }

    #[test]
    fn test_example5() {
        assert!(
            Solution::increasing_triplet(vec![1, 7, 3, 2, 4]),
        );
    }

    #[test]
    fn test_example6() {
        assert!(
            Solution::increasing_triplet(vec![1, 2, -10, -8, -7]),
        );
    }

    #[test]
    fn test_example7() {
        assert!(
            Solution::increasing_triplet(vec![1, 0, 0, 0, 10, 0, 0, 0, 100]),
        );
    }
}
