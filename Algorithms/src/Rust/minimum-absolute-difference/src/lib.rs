// https://leetcode.com/problems/minimum-absolute-difference/

struct Solution;

impl Solution {
    pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
        arr.sort();
        let d = (0..arr.len()-1).fold(i32::MAX, |acc, i| {
            acc.min(arr[i+1] - arr[i])
        });

        (0..arr.len()-1).fold(Vec::new(), |mut acc, i| {
            if arr[i+1] - arr[i] == d {
                acc.push(vec![arr[i], arr[i+1]]);
            }
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_abs_difference() {
        assert_eq!(
            vec![vec![1, 2], vec![2, 3], vec![3, 4]],
            Solution::minimum_abs_difference(vec![4, 2, 1, 3])
        );

        assert_eq!(
            vec![vec![1, 3]],
            Solution::minimum_abs_difference(vec![1, 3, 6, 10, 15])
        );

        assert_eq!(
            vec![vec![-14, -10], vec![19, 23], vec![23, 27]],
            Solution::minimum_abs_difference(vec![3, 8, -10, 23, 19, -4, -14, 27])
        );
    }
}
