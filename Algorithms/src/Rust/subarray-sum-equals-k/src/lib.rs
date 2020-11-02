// https://leetcode.com/problems/subarray-sum-equals-k/

pub struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut acc = vec![0; nums.len() + 1];

        for i in 1..=nums.len() {
            acc[i] = acc[i-1] + nums[i-1];
        }

        let mut count: i32 = 0;

        for i in 0..nums.len() {
            for j in i+1..=nums.len() {
                if acc[j] - acc[i] == k {
                    count += 1;
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        nums: Vec<i32>,
        k: i32,
        expected: i32,
    }

    #[test]
    fn test_subarray_sum() {
        let test_cases: [TestCase; 3] = [
            TestCase {
                nums: vec![1, 1, 1],
                k: 2,
                expected: 2,
            },
            TestCase {
                nums: vec![49, -2, 10, 0, 8, -6, 6, 102, -94, 0, 1],
                k: 8,
                expected: 10,
            },
            TestCase {
                nums: vec![0, 1, 2, 3, 4, 5, 6, 0],
                k: 6,
                expected: 4,
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::subarray_sum(case.nums.to_vec(), case.k),
            );
        }
    }
}
