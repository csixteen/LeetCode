// https://leetcode.com/problems/shuffle-the-array/

struct Solution;

impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        (0..n as usize).fold(Vec::new(), |mut acc, i| {
            acc.push(nums[i]);
            acc.push(nums[i+ (n as usize)]);
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Solution::shuffle(vec![2, 5, 1, 3, 4, 7], 3),
            vec![2, 3, 5, 4, 1, 7],
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Solution::shuffle(vec![1, 2, 3, 4, 4, 3, 2, 1], 4),
            vec![1, 4, 2, 3, 3, 2, 4, 1],
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            Solution::shuffle(vec![1, 1, 2, 2], 2),
            vec![1, 2, 1, 2],
        );
    }
}
