// https://leetcode.com/problems/find-the-highest-altitude/

struct Solution;

impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let (m, p) = gain.iter().fold((0, 0), |(m, p), i| (m.max(p), p+i));
        m.max(p)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(1, Solution::largest_altitude(vec![-5, 1, 5, 0, -7]));
    }

    #[test]
    fn example2() {
        assert_eq!(0, Solution::largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]));
    }
}
