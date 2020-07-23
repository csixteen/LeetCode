// https://leetcode.com/problems/merge-sorted-array/

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let (mut m, mut n) = (m as usize, n as usize);

        while n > 0 {
            if m > 0 && nums1[m - 1] > nums2[n - 1] {
                nums1[m + n - 1] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[m + n - 1] = nums2[n - 1];
                n -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];

        Solution::merge(&mut nums1, 3, &mut nums2, 3);

        assert_eq!(vec![1, 2, 2, 3, 5, 6], nums1);
    }

    #[test]
    fn test_example2() {
        let mut nums1 = vec![3, 9, 0, 0, 0, 0];
        let mut nums2 = vec![1, 5, 6, 11];

        Solution::merge(&mut nums1, 2, &mut nums2, 4);

        assert_eq!(vec![1, 3, 5, 6, 9, 11], nums1);
    }
}
