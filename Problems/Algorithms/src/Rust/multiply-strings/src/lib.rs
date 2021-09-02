// https://leetcode.com/problems/multiply-strings/

struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        let mut res = vec![0; num1.len() + num2.len()];

        for (i, x) in num1.chars().rev().enumerate() {
            for (j, y) in num2.chars().rev().enumerate() {
                let a = x.to_digit(10).unwrap();
                let b = y.to_digit(10).unwrap();

                let m = a * b + res[i + j];
                let carry = m / 10;

                res[i + j] = m % 10;
                res[i + j + 1] += carry;
            }
        }

        while res.len() > 1 && res.last() == Some(&0) {
            res.pop();
        }

        String::from_utf8(res.iter().rev().map(|d| (*d as u8) + b'0').collect()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            "6".to_string(),
            Solution::multiply("2".to_string(), "3".to_string()),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            "56088".to_string(),
            Solution::multiply("123".to_string(), "456".to_string()),
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            "998001".to_string(),
            Solution::multiply("999".to_string(), "999".to_string()),
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            "0".to_string(),
            Solution::multiply("123".to_string(), "0".to_string()),
        );
    }

    #[test]
    fn example5() {
        assert_eq!(
            "14280".to_string(),
            Solution::multiply("102".to_string(), "140".to_string()),
        );
    }

    #[test]
    fn example6() {
        assert_eq!(
            "0".to_string(),
            Solution::multiply("0".to_string(), "52".to_string()),
        );
    }

    #[test]
    fn example7() {
        assert_eq!(
            "2324725235680339589575434145174345450376468286967007130548581359508676923461769510883584014763890133705678997934".to_string(),
            Solution::multiply(
                "6964594125027226699998128707627435007621143975736747759751".to_string(),
                "333791918659904899647584436187733004125181273682766434".to_string(),
            ),
        );
    }
}
