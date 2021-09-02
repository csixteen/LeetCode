// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/

struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        if num == 0 { return 0 }
        (num.count_ones() + (32 - num.leading_zeros()) - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(6, Solution::number_of_steps(14));
    }

    #[test]
    fn example2() {
        assert_eq!(4, Solution::number_of_steps(8));
    }

    #[test]
    fn example3() {
        assert_eq!(12, Solution::number_of_steps(123));
    }

    #[test]
    fn example4() {
        assert_eq!(0, Solution::number_of_steps(0));
    }
}
