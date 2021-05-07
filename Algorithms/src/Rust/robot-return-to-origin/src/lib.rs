// https://leetcode.com/problems/robot-return-to-origin/

struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        moves.chars().fold((0, 0), |acc, c|
            match c {
                'U' => (acc.0, acc.1 + 1),
                'D' => (acc.0, acc.1 - 1),
                'L' => (acc.0 - 1, acc.1),
                'R' => (acc.0 + 1, acc.1),
                _ => panic!(),
            }
        ) == (0,0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_judge_circle() {
        assert!(Solution::judge_circle("UD".to_string()));
        assert!(!Solution::judge_circle("LL".to_string()));
        assert!(!Solution::judge_circle("RRDD".to_string()));
        assert!(!Solution::judge_circle("LDRRLRUULR".to_string()));
    }
}
