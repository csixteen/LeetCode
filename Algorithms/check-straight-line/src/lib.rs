// https://leetcode.com/problems/check-if-it-is-a-straight-line/

struct Solution {}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        fn slope(a: &Vec<i32>, b: &Vec<i32>) -> f32 {
            (b[1] as f32 - a[1] as f32) / (b[0] as f32 - a[0] as f32)
        }

        if coordinates.len() < 3 {
            return true;
        }

        let s = slope(&coordinates[0], &coordinates[1]);

        for i in 2..coordinates.len() {
            if slope(&coordinates[i-1], &coordinates[i]) != s {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        coordinates: Vec<Vec<i32>>,
        expected: bool,
    }

    #[test]
    fn test_check_straight_line() {
        let test_cases = [
            TestCase {
                coordinates: vec![
                    vec![1, 2],
                    vec![2, 3],
                    vec![3, 4],
                    vec![4, 5],
                    vec![5, 6],
                    vec![6, 7],
                ],
                expected: true,
            },
            TestCase {
                coordinates: vec![
                    vec![1, 1],
                    vec![2, 2],
                    vec![3, 4],
                    vec![4, 5],
                    vec![5, 6],
                    vec![7, 7],
                ],
                expected: false,
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::check_straight_line(case.coordinates.clone()),
            );
        }
    }
}
