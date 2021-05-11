// https://leetcode.com/problems/monotonic-array/

struct Solution;

impl Solution {
    pub fn is_monotonic(a: Vec<i32>) -> bool {
        let mut prev = 0;

        for i in 1..a.len() {
            let s = (a[i] - a[i-1]).signum();
            if s == 0 {
                continue;
            } else if prev == 0 {
                prev = s;
            } else if s != prev {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_monotonic() {
        assert!(Solution::is_monotonic(vec![1, 2, 2, 3]));
        assert!(Solution::is_monotonic(vec![6, 5, 4, 4]));
        assert!(!Solution::is_monotonic(vec![1, 3, 2]));
        assert!(Solution::is_monotonic(vec![1, 2, 4, 5]));
        assert!(Solution::is_monotonic(vec![1, 1, 1]));
        assert!(!Solution::is_monotonic(
            vec![11, 11, 9, 4, 3, 3, 3, 1, -1, -1, 3, 3, 3, 5, 5, 5]
        ));
    }
}
