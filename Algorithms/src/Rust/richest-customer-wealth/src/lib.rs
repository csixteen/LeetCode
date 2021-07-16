// https://leetcode.com/problems/richest-customer-wealth/

struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts
            .iter()
            .map(|i| i.iter().sum())
            .max()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(6, Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]));
    }

    #[test]
    fn example2() {
        assert_eq!(10, Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]));
    }

    #[test]
    fn example3() {
        assert_eq!(
            17,
            Solution::maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]),
        );
    }
}
