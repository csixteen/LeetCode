// https://leetcode.com/problems/slowest-key/

use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn slowest_key(release_times: Vec<i32>, keys_pressed: String) -> char {
        let mut res = keys_pressed.chars().nth(0).unwrap();
        let mut slowest = release_times[0];

        for (i, c) in keys_pressed.chars().enumerate().skip(1) {
            let diff = release_times[i] - release_times[i - 1];
            match diff.cmp(&slowest) {
                Ordering::Greater => {
                    res = c;
                    slowest = diff;
                }
                Ordering::Equal if c > res => res = c,
                _ => (),
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slowest_key() {
        assert_eq!(
            'c',
            Solution::slowest_key(vec![9, 29, 49, 50], String::from("cbcd"))
        );
        assert_eq!(
            'a',
            Solution::slowest_key(vec![12, 23, 36, 46, 62], String::from("spuda"))
        );
    }
}
