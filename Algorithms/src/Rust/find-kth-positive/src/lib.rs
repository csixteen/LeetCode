// https://leetcode.com/problems/kth-missing-positive-number/

struct Solution;

impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        // Time complexity: O(N)
        let mut missing = 0;
        let mut previous = 0;
        let mut k = k;

        for n in arr.iter() {
            let d = k.min(n - previous - 1);
            missing = previous + d;
            k -= d;
            previous = *n;
            if k == 0 { break; }
        }

        if k == 0 {
            missing
        } else {
            previous + k
        }
    }

    pub fn find_kth_positive_simplified(arr: Vec<i32>, k: i32) -> i32 {
        // Time complexity: O(N)
        let mut i = 0;
        let mut k = k;

        while i < arr.len() && arr[i] <= k {
            k += 1;
            i += 1;
        }

        k
    }

    pub fn find_kth_positive_bs(arr: Vec<i32>, k: i32) -> i32 {
        let len = arr.len();
        let (mut lo, mut hi) = (0, len - 1);

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;

            if k > arr[mid] - (mid as i32) - 1 {
                 lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        lo as i32 + k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_kth_positive() {
        assert_eq!(9, Solution::find_kth_positive(vec![2, 3, 4, 7, 11], 5));
        assert_eq!(6, Solution::find_kth_positive(vec![1, 2, 3, 4], 2));
    }

    #[test]
    fn find_kth_positive_simplified() {
        assert_eq!(9, Solution::find_kth_positive_simplified(vec![2, 3, 4, 7, 11], 5));
        assert_eq!(6, Solution::find_kth_positive_simplified(vec![1, 2, 3, 4], 2));
    }

    #[test]
    fn find_kth_positive_bs() {
        assert_eq!(9, Solution::find_kth_positive_bs(vec![2, 3, 4, 7, 11], 5));
        assert_eq!(6, Solution::find_kth_positive_bs(vec![1, 2, 3, 4], 2));
    }
}
