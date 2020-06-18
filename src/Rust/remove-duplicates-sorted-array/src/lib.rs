// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

struct Solution {}

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() { return 0 }

        let mut slow = 0usize;
        let mut fast = 1usize;

        while fast < nums.len() {
            if nums[fast] != nums[slow] {
                slow += 1;
                nums[slow] = nums[fast];
                fast += 1;
            } else {
                fast += 1;
            }
        }

        slow as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut nums = vec![1, 1, 2];
        let res = Solution::remove_duplicates(&mut nums);
        assert_eq!(2, res);
        assert_eq!(vec![1, 2, 2], nums);
    }

    #[test]
    fn test_example2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let res = Solution::remove_duplicates(&mut nums);
        assert_eq!(5, res);
        assert_eq!(vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4], nums);
    }

    #[test]
    fn test_example3() {
        let mut nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
        let res = Solution::remove_duplicates(&mut nums);
        assert_eq!(1, res);
        assert_eq!(vec![1, 1, 1, 1, 1, 1, 1, 1], nums);
    }

    #[test]
    fn test_example4() {
        let mut nums = vec![0, 0, 0, 0, 1];
        let res = Solution::remove_duplicates(&mut nums);
        assert_eq!(2, res);
        assert_eq!(vec![0, 1, 0, 0, 1], nums);
    }
}
