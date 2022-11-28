use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, i32> = HashMap::new();
        for (index, elem) in nums.iter().enumerate() {
            match seen.get(&(target - elem)) {
                Some(&i) => return vec![i, index as i32],
                None    => seen.insert(*elem, index as i32),
            };
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
    }

    #[test]
    fn example2() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3, 2, 4], 6));
    }

    #[test]
    fn example3() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![3, 3], 6));
    }
}
