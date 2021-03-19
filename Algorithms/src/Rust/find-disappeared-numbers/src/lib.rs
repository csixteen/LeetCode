// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/

struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();

        for j in 0..len {
            let i = (nums[j].abs() - 1) as usize;
            if nums[i] > 0 {
                nums[i] *= -1;
            }
        }

        (0..len).fold(Vec::new(), |mut acc, i| {
            if nums[i] > 0 {
                acc.push((i + 1) as i32);
            }
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_disappeared_numbers() {
        assert_eq!(
            vec![5, 6],
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );
    }
}
