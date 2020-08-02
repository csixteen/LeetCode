// https://leetcode.com/problems/pascals-triangle/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::new();

        for i in 0..num_rows as usize {
            ret.push((1..i).fold(vec![1; (i + 1) as usize], |mut acc, j| {
                acc[j] = ret[i-1][j-1] + ret[i-1][j];
                acc
            }));
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1],
            ],
            Solution::generate(5),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(vec![vec![1]], Solution::generate(1));
    }

    #[test]
    fn test_example3() {
        assert_eq!(Vec::<Vec<i32>>::new(), Solution::generate(0));
    }
}
