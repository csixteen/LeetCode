// https://leetcode.com/problems/sum-of-all-odd-length-subarrays/

struct Solution;

impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        fn go(xs: &Vec<i32>, lo: usize, hi: usize, acc: i32) -> i32 {
            if hi > xs.len() { return acc }
            else {
                go(xs, lo, hi+2, acc + &xs[lo..hi].iter().sum())
            }
        }

        (0..arr.len()).fold(0, |acc, i| acc + go(&arr, i, i+1, 0))
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_sum_odd_length_subarrays() {
        assert_eq!(58, Solution::sum_odd_length_subarrays(vec![1, 4, 2, 5, 3]));
        assert_eq!(3, Solution::sum_odd_length_subarrays(vec![1, 2]));
        assert_eq!(66, Solution::sum_odd_length_subarrays(vec![10, 11, 12]));
    }
}
