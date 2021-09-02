struct Solution {}

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        fn bin_search(n: &Vec<i32>, lo: usize, hi: usize) -> i32 {
            if lo == hi {
                return n[lo];
            }

            let mid = lo + ((hi - lo) / 2) as usize;

            match mid % 2 {
                0 if n[mid] == n[mid-1] => { return bin_search(n, lo, mid-2); },
                0 if n[mid] == n[mid+1] => { return bin_search(n, mid+2, hi); },
                1 if n[mid] == n[mid-1] => { return bin_search(n, mid+1, hi); },
                1 if n[mid] == n[mid+1] => { return bin_search(n, lo, mid-1); },
                _ => n[mid],
            }
        }

        bin_search(&nums, 0, nums.len() - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        nums: Vec<i32>,
        expected: i32,
    }

    #[test]
    fn test_single_non_duplicate() {
        let test_cases = [
            TestCase {
                nums: vec![1, 1, 2, 3, 3, 4, 4, 8, 8],
                expected: 2,
            },
            TestCase {
                nums: vec![3, 3, 7, 7, 10, 11, 11],
                expected: 10,
            },
            TestCase {
                nums: vec![1, 2, 2],
                expected: 1,
            },
            TestCase {
                nums: vec![1, 1, 2],
                expected: 2,
            },
            TestCase {
                nums: vec![1, 1, 2, 2, 5, 5, 6, 6, 7, 8, 8, 9, 9],
                expected: 7,
            },
            TestCase {
                nums: vec![1, 1, 2, 3, 3],
                expected: 2,
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::single_non_duplicate(case.nums.clone()),
            );
        }
    }
}
