// https://leetcode.com/problems/interval-list-intersections/

use std::cmp::Ordering;
use std::collections::VecDeque;

struct Solution {}

type Interval = Vec<i32>;

impl Solution {
    pub fn interval_intersection(a: Vec<Interval>, b: Vec<Interval>) -> Vec<Interval> {
        let mut a = VecDeque::from(a);
        let mut b = VecDeque::from(b);
        let mut ret = Vec::new();

        while !a.is_empty() && !b.is_empty() {
            if a[0][1] < b[0][0] { a.pop_front(); continue; }
            else if b[0][1] < a[0][0] { b.pop_front(); continue; }

            ret.push(vec![a[0][0].max(b[0][0]), a[0][1].min(b[0][1])]);

            match a[0][1].cmp(&b[0][1]) {
                Ordering::Less => {
                    b[0][0] = a[0][0];
                    a.pop_front();
                },
                Ordering::Greater => {
                    a[0][0] = b[0][0];
                    b.pop_front();
                },
                Ordering::Equal => {
                    a.pop_front();
                    b.pop_front();
                },
            }
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
                vec![1, 2],
                vec![5, 5],
                vec![8, 10],
                vec![15, 23],
                vec![24, 24],
                vec![25, 25]
            ],
            Solution::interval_intersection(
                vec![vec![0, 2], vec![5, 10], vec![13, 23], vec![24, 25]],
                vec![vec![1, 5], vec![8, 12], vec![15, 24], vec![25, 26]],
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            vec![vec![2, 5], vec![7, 9], vec![10, 12], vec![16, 17]],
            Solution::interval_intersection(
                vec![vec![1, 5], vec![7, 15], vec![16, 17]],
                vec![vec![2, 9], vec![10, 12], vec![16, 18]],
            ),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            vec![vec![2, 3], vec![4, 5], vec![6, 7]],
            Solution::interval_intersection(
                vec![vec![0, 7]],
                vec![vec![2, 3], vec![4, 5], vec![6, 7]],
            ),
        );
    }

    #[test]
    fn test_example4() {
        assert_eq!(
            vec![vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6]],
            Solution::interval_intersection(
                vec![vec![1, 2], vec![3, 4], vec![5, 6]],
                vec![vec![2, 3], vec![4, 5], vec![6, 7]],
            ),
        );
    }
}
