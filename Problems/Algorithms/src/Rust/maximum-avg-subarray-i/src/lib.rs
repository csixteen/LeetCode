// https://leetcode.com/problems/maximum-average-subarray-i/

struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut curr = nums.iter().take(k as usize).sum::<i32>();
        let mut max = curr;

        for j in (k as usize)..nums.len() {
            curr += nums[j] - nums[j - k as usize];
            max = max.max(curr);
        }

        max as f64 / k as f64
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(12.75, Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4));
    }

    #[test]
    fn example2() {
        assert_eq!(5.0, Solution::find_max_average(vec![5], 1));
    }
}
