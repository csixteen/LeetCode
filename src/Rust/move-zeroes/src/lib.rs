// https://leetcode.com/problems/move-zeroes/

struct Solution {}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let len = nums.len();
        let mut last_non_zero = 0;

        for i in 0..len {
            if nums[i] != 0 {
                nums[last_non_zero] = nums[i];
                last_non_zero += 1;
            }
        }

        for i in last_non_zero..len {
            nums[i] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut v = vec![0, 1, 0, 3, 12];

        Solution::move_zeroes(&mut v);

        assert_eq!(v, vec![1, 3, 12, 0, 0]);
    }
}
