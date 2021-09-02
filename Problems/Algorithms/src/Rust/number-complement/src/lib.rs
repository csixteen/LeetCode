struct Solution {}

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        num ^ (2_i32.pow(32 - num.leading_zeros()) - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        num: i32,
        expected: i32,
    }

    #[test]
    fn test_find_complement() {
        let test_cases = [
            TestCase {
                num: 5,
                expected: 2,
            },
            TestCase {
                num: 1,
                expected: 0,
            },
            TestCase {
                num: 10,
                expected: 5,
            },
            TestCase {
                num: 20,
                expected: 11,
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::find_complement(case.num),
            );
        }
    }
}
