// https://leetcode.com/problems/longest-continuous-increasing-subsequence/

struct Solution;

impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        nums.iter().skip(1).fold((nums[0], 1, 1), |(prev, acc, max), i| {
            let acc = if *i > prev { acc + 1 } else { 1 };
            let max = acc.max(max);
            let prev = i;
            (*prev, acc, max)
        }).2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(3, Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]));
    }

    #[test]
    fn example2() {
        assert_eq!(1, Solution::find_length_of_lcis(vec![2, 2, 2, 2, 2, 2]));
    }
}
