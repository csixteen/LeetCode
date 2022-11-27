struct Solution;

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut slow, mut fast) = (0usize, 1usize);

        while fast < nums.len() {
            if nums[slow] != 0 {
                slow += 1;
            } else if nums[fast] != 0 {
                nums.swap(slow, fast);
                slow += 1;
            }
            fast += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut v = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![1, 3, 12, 0, 0], v);
    }

    #[test]
    fn example2() {
        let mut v = vec![0];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![0], v);
    }

    #[test]
    fn example3() {
        let mut v = vec![1, 2, 3, 4, 5];
        Solution::move_zeroes(&mut v);
        assert_eq!(vec![1, 2, 3, 4, 5], v);
    }
}
