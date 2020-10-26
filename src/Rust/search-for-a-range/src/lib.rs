// https://leetcode.com/explore/interview/card/top-interview-questions-medium/110/sorting-and-searching/802/

struct Solution;

impl Solution {
    pub fn search_range_linear(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = vec![-1, -1];

        res[0] = match nums.iter().position(|&x| x == target) {
            Some(p) => p as i32,
            None => -1,
        };

        res[1] = match nums.iter().rev().position(|&x| x == target) {
            Some(p) => (nums.len() - 1 - p) as i32,
            None => -1,
        };

        res
    }

    pub fn serch_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        nums: Vec<i32>,
        target: i32,
        expected: Vec<i32>,
    }

    #[test]
    fn test_search_range() {
        let tests = vec![
            TestCase{
                nums: vec![5, 7, 7, 8, 8, 10],
                target: 8,
                expected: vec![3, 4],
            },
            TestCase{
                nums: vec![5, 7, 7, 8, 8, 10],
                target: 6,
                expected: vec![-1, -1],
            },
            TestCase{
                nums: vec![],
                target: 0,
                expected: vec![-1, -1],
            }
        ];

        for t in tests.iter() {
            assert_eq!(
                Solution::search_range_linear(t.nums.clone(), t.target),
                t.expected.clone(),
            );
        }
    }
}
