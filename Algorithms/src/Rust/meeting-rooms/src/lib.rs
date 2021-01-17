// https://leetcode.com/problems/meeting-rooms/

struct Solution;

impl Solution {
    pub fn can_attend_meetings(intervals: Vec<Vec<i32>>) -> bool {
        if intervals.len() <= 1 { return true }

        let mut int = intervals.clone();
        
        &int.sort_by(|a, b| a[0].cmp(&b[0]));

        for i in 0..int.len()-1 {
            if int[i+1][0] < int[i][1] {
                return false
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert!(!Solution::can_attend_meetings(vec![vec![0, 30], vec![5, 10], vec![15, 20]]));
    }

    #[test]
    fn test_example2() {
        assert!(Solution::can_attend_meetings(vec![vec![7, 10], vec![2, 4]]));
    }
}
