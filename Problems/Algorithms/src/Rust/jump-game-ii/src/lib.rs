// https://leetcode.com/problems/jump-game-ii/

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut max_jump = 0;
        let mut jump_end = 0;
        let mut total_jumps = 0;

        for i in 0..nums.len() - 1 {
            max_jump = max_jump.max(i + nums[i] as usize);
            if i == jump_end {
                jump_end = max_jump;
                total_jumps += 1;
            }
        }

        total_jumps
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
    }

    #[test]
    fn example2() {
        assert_eq!(2, Solution::jump(vec![2, 3, 0, 1, 4]));
    }
}
