// https://leetcode.com/problems/sort-array-by-parity/

struct Solution;

impl Solution {
    pub fn sort_array_by_parity(mut a: Vec<i32>) -> Vec<i32> {
        let (mut i, mut j) = (0, a.len() - 1);
        while i < j {
            if a[i] % 2 != 0 {
                a.swap(i, j);
                j -= 1;
            } else {
                i += 1;
            }
        }
        
        a
    }

    pub fn sort_array_by_parity2(a: Vec<i32>) -> Vec<i32> {
        let (even, odd): (Vec<i32>, Vec<i32>) = a.iter().partition(|&n| n % 2 == 0);
        [even, odd].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_array_by_parity() {
        assert_eq!(
            vec![2, 4, 3, 1],
            Solution::sort_array_by_parity(vec![3, 1, 2, 4])
        );
    }

    #[test]
    fn test_sort_array_by_parity2() {
        assert_eq!(
            vec![2, 4, 3, 1],
            Solution::sort_array_by_parity2(vec![3, 1, 2, 4])
        );
    }
}
