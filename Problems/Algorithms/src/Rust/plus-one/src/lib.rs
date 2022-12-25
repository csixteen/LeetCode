// https://leetcode.com/problems/plus-one/

#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;

        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            } else {
                digits[i] = 0;
            }
        }

        digits.insert(0, 1);

        digits
    }

    pub fn plus_one_2(digits: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        let mut carry: i32 = 1;
        
        for i in digits.iter().rev() {
            let x = (*i + carry) % 10;
            res.push(x);

            if x > 0 { carry = 0; }
        }

        if carry > 0 { res.push(1); }

        res.iter().rev().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![1, 2, 4],
            Solution::plus_one(vec![1, 2, 3]),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            vec![4, 3, 2, 2],
            Solution::plus_one(vec![4, 3, 2, 1]),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            vec![1, 0, 0, 0],
            Solution::plus_one(vec![9, 9, 9]),
        );
    }
}
