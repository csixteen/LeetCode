// https://leetcode.com/problems/find-the-town-judge/

#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![0; n as usize];
        let mut trust_counter = vec![0; n as usize];

        for v in trust.iter() {
            graph[(v[0] - 1) as usize] += 1;
            trust_counter[(v[1] - 1) as usize] += 1;
        }

        for i in 0..n {
            if trust_counter[i as usize] == (n - 1) as usize
                && graph[i as usize] == 0 {
                return i + 1;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        n: i32,
        trust: Vec<Vec<i32>>,
        expected: i32,
    }

    #[test]
    fn test_find_judge() {
        let test_cases = [
            TestCase {
                n: 2,
                trust: vec![vec![1, 2]],
                expected: 2,
            },
            TestCase {
                n: 3,
                trust: vec![vec![1, 3], vec![2, 3]],
                expected: 3,
            },
            TestCase {
                n: 3,
                trust: vec![vec![1, 3], vec![2, 3], vec![3, 1]],
                expected: -1,
            },
            TestCase {
                n: 3,
                trust: vec![vec![1, 2], vec![2, 3]],
                expected: -1,
            },
            TestCase {
                n: 4,
                trust: vec![
                    vec![1, 3],
                    vec![1, 4],
                    vec![2, 3],
                    vec![2, 4],
                    vec![4, 3],
                ],
                expected: 3,
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::find_judge(case.n, case.trust.clone()),
            );
        }
    }
}
