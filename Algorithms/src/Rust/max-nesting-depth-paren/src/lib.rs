// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/

struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        s.chars().fold((0, 0), |acc, c| {
            match c {
                '(' => (((acc.1+1).max(acc.0)), acc.1+1),
                ')' => (acc.0, acc.1-1),
                _ => acc
            }
        }).0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_depth() {
        assert_eq!(3, Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()));
        assert_eq!(3, Solution::max_depth("(1)+((2))+(((3)))".to_string()));
        assert_eq!(1, Solution::max_depth("1+(2*3)/(2-1)".to_string()));
        assert_eq!(0, Solution::max_depth("1".to_string()));
    }
}
