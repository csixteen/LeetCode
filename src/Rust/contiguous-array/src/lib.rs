// https://leetcode.com/problems/contiguous-array/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut m = 0;
        let mut count = 0;
        let mut hash: HashMap<i32, i32> = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            count += if *n == 1 { 1 } else { -1 };

            if count == 0 { 
                m = m.max(i + 1);
            } else if hash.contains_key(&count) {
                m = m.max(i - *hash.get(&count).unwrap() as usize);
            } else {
                hash.insert(count, i as i32);
            }
        }

        m as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            2,
            Solution::find_max_length(vec![0, 1]),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            2,
            Solution::find_max_length(vec![0, 1, 0]),
        );
    }
}
