// https://leetcode.com/problems/count-binary-substrings/

struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let mut prev = 'x';
        let mut groups = Vec::new();

        for c in s.chars() {
            if prev == c {
                let l = groups.len();
                groups[l - 1] += 1;
            } else {
                groups.push(1);
            }

            prev = c;
        }

        groups
            .iter()
            .zip(groups.iter().skip(1))
            .map(|(a, b)| a.min(b))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_binary_substrings() {
        assert_eq!(
            6,
            Solution::count_binary_substrings(String::from("00110011"))
        );
        assert_eq!(4, Solution::count_binary_substrings(String::from("10101")));
    }
}
