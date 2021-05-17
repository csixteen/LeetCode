// https://leetcode.com/problems/min-cost-climbing-stairs/

use std::collections::HashMap;

struct Solution;

impl Solution {
    // Iterative bottom-up with O(N) space
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let len = cost.len() + 1;
        let mut ret = vec![0_i32; len];
        for i in 2..len {
            ret[i] = (cost[i-2] + ret[i-2]).min(cost[i-1] + ret[i-1]);
        }

        ret[len-1]
    }

    // Recursive top-down with memoization (O(N)) space
    pub fn min_cost_climbing_stairs2(cost: Vec<i32>) -> i32 {
        fn go(xs: &Vec<i32>, i: usize, acc: &mut HashMap<usize, i32>) -> i32 {
            if i <= 1 { return 0 }

            let one = xs[i-1] + go(xs, i-1, acc);
            let two = xs[i-2] + go(xs, i-2, acc);
            let res = one.min(two);
            acc.insert(i, res);

            res
        }

        let mut cache = HashMap::new();
        go(&cost, cost.len(), &mut cache)
    }

    // Iterative bottom-up with constant space
    pub fn min_cost_climbing_stairs3(cost: Vec<i32>) -> i32 {
        let (mut a, mut b) = (0, 0);
        for i in 2..=cost.len() {
            let c = a;
            a = (a + cost[i-1]).min(b + cost[i-2]);
            b = c;
        }

        a
    }

    // Recursive bottom-up with constant space
    pub fn min_cost_climbing_stairs4(cost: Vec<i32>) -> i32 {
        fn go(xs: &Vec<i32>, i: usize, a: i32, b: i32) -> i32 {
            if i > xs.len() {
                a
            } else {
                go(xs, i+1, (a + xs[i-1]).min(b + xs[i-2]), a)
            }
        }

        go(&cost, 2, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_cost_climbing_stairs() {
        assert_eq!(15, Solution::min_cost_climbing_stairs(vec![10, 15, 20]));
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }

    #[test]
    fn test_min_cost_climbing_stairs2() {
        assert_eq!(15, Solution::min_cost_climbing_stairs2(vec![10, 15, 20]));
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs2(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }

    #[test]
    fn test_min_cost_climbing_stairs3() {
        assert_eq!(15, Solution::min_cost_climbing_stairs3(vec![10, 15, 20]));
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs3(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }

    #[test]
    fn test_min_cost_climbing_stairs4() {
        assert_eq!(15, Solution::min_cost_climbing_stairs4(vec![10, 15, 20]));
        assert_eq!(
            6,
            Solution::min_cost_climbing_stairs4(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
        );
    }
}
