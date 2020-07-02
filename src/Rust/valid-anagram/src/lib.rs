// https://leetcode.com/problems/valid-anagram/

struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let s_freq = s.bytes().fold([0; 26], |mut acc, c| {
            acc[(c - b'a') as usize] += 1;
            acc
        });

        let t_freq = t.bytes().fold([0; 26], |mut acc, c| {
            acc[(c - b'a') as usize] += 1;
            acc
        });

        for i in 0..26 {
            if s_freq[i] != t_freq[i] {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert!(
            Solution::is_anagram(
                String::from("anagram"),
                String::from("nagaram"),
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert!(
            !Solution::is_anagram(
                String::from("rat"),
                String::from("cat"),
            ),
        );
    }
}
