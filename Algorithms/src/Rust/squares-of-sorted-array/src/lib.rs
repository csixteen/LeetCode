// https://leetcode.com/problems/squares-of-a-sorted-array/submissions/

struct Solution;

impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut res = vec![0; n];
        let (mut left, mut right) = (0, n - 1);

        let mut square: i32;

        for i in 0..n {
            if nums[left].abs() > nums[right].abs() {
                square = nums[left];
                left += 1;
            } else {
                square = nums[right];
                right -= 1;
            }

            res[n - i - 1] = square * square;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_squares() {
        assert_eq!(
            vec![0, 1, 9, 16, 100],
            Solution::sorted_squares(vec![-4, -1, 0, 3, 10])
        );

        assert_eq!(
            vec![4, 9, 9, 49, 121],
            Solution::sorted_squares(vec![-7, -3, 2, 3, 11])
        );

        assert_eq!(
            vec![1, 4, 9, 16, 25],
            Solution::sorted_squares(vec![-5, -4, -3, -2, -1])
        );
    }
}
