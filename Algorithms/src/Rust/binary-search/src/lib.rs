// https://leetcode.com/problems/binary-search/

struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut lo, mut hi) = (0, nums.len());

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            match target.cmp(&nums[mid]) {
                Ordering::Equal => return mid as i32,
                Ordering::Less => hi = mid,
                Ordering::Greater => lo = mid + 1,
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_search() {
        assert_eq!(4, Solution::search(vec![-1, 0, 3, 5, 9, 12], 9));
        assert_eq!(-1, Solution::search(vec![-1, 0, 3, 5, 9, 12], 2));
        assert_eq!(-1, Solution::search(vec![5], -5));
        assert_eq!(0, Solution::search(vec![5], 5));
    }
}
