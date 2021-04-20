// https://leetcode.com/problems/find-common-characters/

struct Solution;

impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let occur = a.iter().enumerate().fold(vec![vec![0; a.len()]; 26], |mut acc, (i, s)| {
            s.bytes().for_each(|c| { acc[(c - b'a') as usize][i] += 1; });
            acc
        });

        occur
            .iter()
            .map(|v| v.iter().min().unwrap())
            .enumerate()
            .filter(|(i, c)| **c > 0)
            .map(|(i, c)| [(b'a' + (i as u8)) as char].repeat(*c))
            .flatten()
            .map(|c| c.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_chars() {
        assert_eq!(
            vec!["e".to_string(), "l".to_string(), "l".to_string()],
            Solution::common_chars(
                vec!["bella".to_string(), "label".to_string(), "roller".to_string()]
            )
        );

        assert_eq!(
            vec!["c".to_string(), "o".to_string()],
            Solution::common_chars(
                vec!["cool".to_string(), "lock".to_string(), "cook".to_string()]
            )
        );
    }
}
