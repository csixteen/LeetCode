struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        nums.iter().skip(1).fold(nums[0], |acc, e| e ^ acc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
    }

    #[test]
    fn example2() {
        assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
    }

    #[test]
    fn example3() {
        assert_eq!(1, Solution::single_number(vec![1]));
    }
}
