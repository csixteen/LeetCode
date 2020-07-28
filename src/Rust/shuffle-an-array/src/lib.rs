// https://leetcode.com/problems/shuffle-an-array/

use rand::{thread_rng, Rng};

struct Solution {
    original: Vec<i32>,
}

impl Solution {
    fn new(nums: Vec<i32>) -> Self {
        Solution { original: nums.clone() }
    }

    fn reset(&self) -> Vec<i32> {
        self.original.clone()
    }

    fn shuffle(&self) -> Vec<i32> {
        let mut rng = thread_rng();
        let mut ret = self.original.clone();

        for i in 0..ret.len() {
            let j = rng.gen_range(0, ret.len());
            ret.swap(i, j);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(one: Vec<i32>, another: Vec<i32>) -> bool {
        let mut another = another;
        &another.sort_unstable();
        one == another
    }

    #[test]
    fn test_example1() {
        let obj = Solution::new(vec![1, 2, 3, 4, 5]);

        assert_eq!(vec![1, 2, 3, 4 ,5], obj.reset());

        assert!(is_valid(vec![1, 2, 3, 4, 5], obj.shuffle()));
        assert!(is_valid(vec![1, 2, 3, 4, 5], obj.shuffle()));
        assert!(is_valid(vec![1, 2, 3, 4, 5], obj.shuffle()));
    }
}
