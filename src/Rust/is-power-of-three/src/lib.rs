// https://leetcode.com/problems/power-of-three/solution/

struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n < 1 { return false; }

        let mut n = n;

        while n % 3 == 0 {
            n = n / 3;
        }

        n == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        val: i32,
        expected: bool,
    }

    const TEST_CASES: [TestCase; 4] = [
        TestCase { val: 27, expected: true },
        TestCase { val: 0, expected: false },
        TestCase { val: 9, expected: true },
        TestCase { val: 2, expected: false },
    ];

    #[test]
    fn test_approach1() {
        for tc in &TEST_CASES {
            assert_eq!(
                tc.expected,
                Solution::is_power_of_three(tc.val),
            );
        }
    }
}
