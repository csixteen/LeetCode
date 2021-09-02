// https://leetcode.com/problems/missing-number/

struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let occurs = nums
            .iter()
            .fold(vec![false; nums.len() + 1], |mut acc, &n| {
                acc[n as usize] = true;
                acc
            });

        occurs.iter().position(|&o| !o).unwrap() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            2,
            Solution::missing_number(vec![3, 0, 1]),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            8,
            Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]),
        );
    }
}
