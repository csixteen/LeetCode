// https://leetcode.com/problems/sort-colors/

#![allow(dead_code)]

struct Solution;

const COLOR_A: i32 = 0;
const COLOR_B: i32 = 1;
const COLOR_C: i32 = 2;

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut a_counter: usize = 0;
        let mut b_counter: usize = 0;
        let mut c_counter: usize = 0;

        nums.iter().for_each(|x| {
            match *x {
                COLOR_A => a_counter += 1,
                COLOR_B => b_counter += 1,
                COLOR_C => c_counter += 1,
                _ => (),
            }
        });

        for i in 0..nums.len() {
            if a_counter > 0 { nums[i] = COLOR_A; a_counter -= 1; }
            else if b_counter > 0 { nums[i] = COLOR_B; b_counter -= 1; }
            else { nums[i] = COLOR_C; c_counter -= 1; }
        }
    }

    pub fn sort_colors_one_pass(nums: &mut Vec<i32>) {
        // Also known as the Dutch flag algorithm
        let mut left: usize = 0;
        let mut middle: usize = 0;
        let mut right: i32 = nums.len() as i32 - 1;

        while middle as i32 <= right {
            match nums[middle] {
                COLOR_A => {
                    &nums.swap(left, middle);
                    left += 1;
                    middle += 1;
                },
                COLOR_C => {
                    &nums.swap(middle, right as usize);
                    right -= 1;
                }
                _ => { middle += 1 },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];

        Solution::sort_colors_one_pass(&mut nums);

        assert_eq!(vec![0, 0, 1, 1, 2, 2], nums);
    }

    #[test]
    fn test_example2() {
        let mut nums = vec![0, 0, 1, 1, 2, 2];

        Solution::sort_colors_one_pass(&mut nums);

        assert_eq!(vec![0, 0, 1, 1, 2, 2], nums);
    }

    #[test]
    fn test_example3() {
        let mut nums = vec![2, 2, 1, 1, 0, 0];

        Solution::sort_colors_one_pass(&mut nums);

        assert_eq!(vec![0, 0, 1, 1, 2, 2], nums);
    }

    #[test]
    fn test_example4() {
        let mut nums = vec![0, 0, 0, 0, 0, 0];

        Solution::sort_colors_one_pass(&mut nums);

        assert_eq!(vec![0, 0, 0, 0, 0, 0], nums);
    }

    #[test]
    fn test_example5() {
        let mut nums = vec![2, 1, 0, 0, 1, 2];

        Solution::sort_colors_one_pass(&mut nums);

        assert_eq!(vec![0, 0, 1, 1, 2, 2], nums);
    }

    #[test]
    fn test_example6() {
        let mut nums = vec![2];

        Solution::sort_colors_one_pass(&mut nums);

        assert_eq!(vec![2], nums);
    }
}
