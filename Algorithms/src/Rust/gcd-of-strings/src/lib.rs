// https://leetcode.com/problems/greatest-common-divisor-of-strings/

struct Solution;

impl Solution {
    fn is_divisor(s: &String, d: &str) -> bool {
        if s.len() % d.len() != 0 { false }
        else {
            s.chars().zip(d.chars().cycle()).find(|&(a, b)| a != b).is_none()
        }
    }

    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        let mut res = 0_usize;
        let mut chars1 = str1.chars();
        let mut chars2 = str2.chars();

        for i in 1..=str1.len().min(str2.len()) {
            match (chars1.next(), chars2.next()) {
                (Some(c1), Some(c2)) =>
                    if c1 != c2 {
                        break;
                    } else if !Self::is_divisor(&str1, &str1[..i]) ||
                        !Self::is_divisor(&str2, &str2[..i]) {
                        continue;
                    } else {
                        res = i;
                    },
                _ => unreachable!(),
            }
        }

        str1.chars().take(res).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_of_strings() {
        assert_eq!(
            String::from("ABC"),
            Solution::gcd_of_strings("ABCABC".to_string(), "ABC".to_string())
        );

        assert_eq!(
            String::from("AB"),
            Solution::gcd_of_strings("ABABAB".to_string(), "ABAB".to_string())
        );

        assert_eq!(
            String::new(),
            Solution::gcd_of_strings("LEET".to_string(), "CODE".to_string())
        );

        assert_eq!(
            String::new(),
            Solution::gcd_of_strings("ABCDEF".to_string(), "ABC".to_string())
        );
    }
}
