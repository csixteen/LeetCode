// https://leetcode.com/problems/set-mismatch/

use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut dup = 0;
        let mut occur: HashSet<i32> = HashSet::new();

        for n in nums.iter() {
            if !occur.insert(*n) {
                dup = *n;
            }
        }

        for i in 1..=nums.len() {
            if !occur.contains(&(i as i32)) {
                return vec![dup, i as i32]
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            vec![2, 3],
            Solution::find_error_nums(vec![1, 2, 2, 4])
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            vec![1, 2],
            Solution::find_error_nums(vec![1, 1])
        );
    }
}
