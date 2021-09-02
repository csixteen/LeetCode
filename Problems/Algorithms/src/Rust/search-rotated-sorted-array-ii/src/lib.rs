// https://leetcode.com/problems/search-in-rotated-sorted-array-ii/

struct Solution;

impl Solution {
    // The worse case time complexity using binary search will be O(N), in
    // case all the elements of the array are the same and we look for
    // something different. So why not just use linear scan? It's simple
    // and it still runs really fast.
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        nums.iter().find(|&&n| n == target).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    }

    #[test]
    fn example2() {
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }

    #[test]
    fn example3() {
        assert!(Solution::search(vec![1, 0, 1, 1, 1], 0));
    }
}
