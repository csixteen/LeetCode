// https://leetcode.com/problems/permutations/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.is_empty() { vec![vec![]] }
        else {
            nums.iter().flat_map(|c| {
                let idx = nums.iter().position(|x| x == c).unwrap() as usize;
                let mut without = nums.clone();
                without.remove(idx);

                Self::permute(without).iter().map(|x| {
                    let mut y = x.clone();
                    y.insert(0, *c);
                    y
                }).collect::<Vec<Vec<i32>>>()
            }).collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_permute() {
        assert_eq!(
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ],
            Solution::permute(vec![1, 2, 3]),
        );
    }
}
