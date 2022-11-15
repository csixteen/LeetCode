use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut seen: HashSet<i32> = HashSet::new();
        for num in nums.iter() {
            if seen.contains(num) {
                return true
            }
            seen.insert(*num);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn example2() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example3() {
        assert!(Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
    }
}
