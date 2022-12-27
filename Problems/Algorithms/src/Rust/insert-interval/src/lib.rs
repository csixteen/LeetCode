// https://leetcode.com/problems/insert-interval/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn insert2(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();

        let mut i = 0;
        while i < intervals.len() && new_interval[0] > intervals[i][1] {
            res.push(intervals[i].clone());
            i += 1;
        }

        let mut j = i;
        while j < intervals.len() && new_interval[1] >= intervals[j][0] { j += 1; }

        if i >= intervals.len() || j == i {
            res.push(new_interval);
        } else {
            res.push(vec![
                intervals[i][0].min(new_interval[0]),
                new_interval[1].max(intervals[j-1][1])
            ]);
        }

        if j < intervals.len() {
            res.append(&mut intervals[j..].to_vec());
        }

        res
    }

    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut less = Vec::new();
        let mut more = Vec::new();
        let mut start = new_interval[0];
        let mut end = new_interval[1];

        for curr in intervals {
            if curr[1] < start {
                less.push(curr);
            } else if curr[0] > end {
                more.push(curr);
            } else {
                start = start.min(curr[0]);
                end = end.max(curr[1]);
            }
        }

        less.push(vec![start, end]);
        less.append(&mut more);
        less
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            vec![vec![1, 5], vec![6, 9]],
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            vec![vec![1, 2], vec![3, 10], vec![12, 16]],
            Solution::insert(
                vec![vec![1, 2], vec![3, 5], vec![6, 7], vec![8, 10], vec![12, 16]],
                vec![4, 8],
            ),
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            vec![vec![5, 7]],
            Solution::insert(vec![], vec![5, 7]),
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            vec![vec![1, 5]],
            Solution::insert(vec![vec![1, 5]], vec![2, 3]),
        );
    }

    #[test]
    fn example5() {
        assert_eq!(
            vec![vec![1, 7]],
            Solution::insert(vec![vec![1, 5]], vec![2, 7]),
        );
    }

    #[test]
    fn example6() {
        assert_eq!(
            vec![vec![0, 0], vec![1, 5]],
            Solution::insert(vec![vec![1, 5]], vec![0, 0]),
        );
    }

    #[test]
    fn example7() {
        assert_eq!(
            vec![vec![0, 5]],
            Solution::insert(vec![vec![1, 5]], vec![0, 2]),
        );
    }
}
