struct Solution {}

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut k = k as usize;
        let mut res = String::with_capacity(num.len());

        for n in num.chars() {
            while k > 0 && !res.is_empty() && n < res.chars().last().unwrap() {
                k -= 1;
                res.pop();
            }

            if !res.is_empty() || n != '0' {
                res.push(n);
            }
        }

        while !res.is_empty() && k > 0 {
            res.pop();
            k -= 1;
        }

        if !res.is_empty() {
            res
        } else {
            String::from("0")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        num: String,
        k: i32,
        expected: String,
    }

    #[test]
    fn test_remove_kdigits() {
        let test_cases = [
            TestCase {
                num: String::from("1432219"),
                k: 3,
                expected: String::from("1219"),
            },
            TestCase {
                num: String::from("10200"),
                k: 1,
                expected: String::from("200"),
            },
            TestCase {
                num: String::from("10"),
                k: 2,
                expected: String::from("0"),
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::remove_kdigits(case.num.clone(), case.k),
            );
        }
    }
}
