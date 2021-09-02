// https://leetcode.com/problems/maximum-product-of-three-numbers/

struct Solution;

impl Solution {
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        let mut min1 = i32::MAX;
        let mut min2: i32 = i32::MAX;
        let mut max1: i32 = i32::MIN;
        let mut max2: i32 = i32::MIN;
        let mut max3: i32 = i32::MIN;

        for num in nums.iter() {
            if *num < min1 {
                min2 = min1;
                min1 = *num;
            } else if *num < min2 {
                min2 = *num;
            }

            if *num > max1 {
                max3 = max2;
                max2 = max1;
                max1 = *num;
            } else if *num > max2 {
                max3 = max2;
                max2 = *num;
            } else if *num > max3 {
                max3 = *num;
            }
        }

        (min1 * min2 * max1).max(max1 * max2 * max3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_product() {
        assert_eq!(6, Solution::maximum_product(vec![1, 2, 3]));
        assert_eq!(24, Solution::maximum_product(vec![1, 2, 3, 4]));
        assert_eq!(-6, Solution::maximum_product(vec![-1, -2, -3]));
    }
}
