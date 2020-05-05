// https://leetcode.com/problems/first-unique-character-in-a-string/

struct Solution {}

impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let char_freq = s
            .bytes()
            .fold([0; 26], |mut acc, c| {
                acc[(c - b'a') as usize] += 1;
                acc
            });

        for (i, c) in s.bytes().enumerate() {
            if char_freq[(c - b'a') as usize] == 1 {
                return i as i32;
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        s: String,
        expected: i32,
    }

    #[test]
    fn test_first_uniq_char() {
        let test_cases = [
            TestCase {
                s: String::from("leetcode"),
                expected: 0,
            },
            TestCase {
                s: String::from("loveleetcode"),
                expected: 2,
            },
            TestCase {
                s: String::from("abcdabcd"),
                expected: -1,
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::first_uniq_char(case.s.clone()),
            );
        }
    }
}
