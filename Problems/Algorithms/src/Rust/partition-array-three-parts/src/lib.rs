// https://leetcode.com/problems/partition-array-into-three-parts-with-equal-sum/

struct Solution;

impl Solution {
    pub fn can_three_parts_equal_sum(arr: Vec<i32>) -> bool {
        let mut acc: i32 = 0;
        let mut parts = 3;
        let sum = arr.iter().sum::<i32>();

        if sum % 3 != 0 { return false }

        let target = sum / 3;

        for i in arr.iter() {
            acc += i;
            if acc == target {
                acc = 0;
                parts -= 1;
            }
        }

        parts < 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_example1() {
        assert!(Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1]));
    }

    #[test]
    fn test_example2() {
        assert!(!Solution::can_three_parts_equal_sum(vec![0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1]));
    }

    #[test]
    fn test_example3() {
        assert!(Solution::can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4]));
    }
}
