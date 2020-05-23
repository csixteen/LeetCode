// https://leetcode.com/problems/ransom-note/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    // First solution. I over engineered, since there is no need to
    // use a HashMap to keep the frequency of chars in each string.
    // This solution runs in 8ms.
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        fn char_counter(s: &String) -> HashMap<char, usize> {
            let mut counter = HashMap::new();

            s.chars().for_each(|c| {
                counter
                    .entry(c)
                    .and_modify(|e| { *e += 1 })
                    .or_insert(1);
            });

            counter
        }

        let ransom_chars = char_counter(&ransom_note);
        let magazine_chars = char_counter(&magazine);

        ransom_chars
            .iter()
            .filter(|(k, v)| {
                !magazine_chars.contains_key(k) || 
                    magazine_chars.get(k).unwrap() < v
            })
            .count() == 0
    }

    // The constraints of the problem make it possible to use a simple
    // array with 26 cells, since we'll only deal with lower case ascii
    // chars. This solution runs in 4ms.
    pub fn can_construct2(ransom_note: String, magazine: String) -> bool {
        fn char_freq(s: &String) -> [u32; 26] {
            s.bytes().fold([0; 26], |mut acc, c| {
                acc[(c - b'a') as usize] += 1;
                acc
            })
        }

        let ransom_char_freq = char_freq(&ransom_note);
        let magazine_char_freq = char_freq(&magazine);
        
        ransom_char_freq
            .iter()
            .zip(magazine_char_freq.iter())
            .all(|(r, m)| r <= m)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        ransom_note: String,
        magazine: String,
        expected: bool,
    }

    #[test]
    fn test_can_construct() {
        let test_cases = [
            TestCase {
                ransom_note: String::from("a"),
                magazine: String::from("b"),
                expected: false,
            },
            TestCase {
                ransom_note: String::from("aa"),
                magazine: String::from("ab"),
                expected: false,
            },
            TestCase {
                ransom_note: String::from("aa"),
                magazine: String::from("aab"),
                expected: true,
            }
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::can_construct(case.ransom_note.clone(), case.magazine.clone()),
            );
            assert_eq!(
                case.expected,
                Solution::can_construct2(case.ransom_note.clone(), case.magazine.clone()),
            );
        }
    }
}
