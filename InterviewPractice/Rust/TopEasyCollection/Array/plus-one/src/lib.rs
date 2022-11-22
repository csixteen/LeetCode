struct Solution;

impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::new();
        let carry = digits.iter().rev().fold(1, |acc, d| {
            res.push((d + acc) % 10);
            (d + acc) / 10
        });

        if carry > 0 {
            res.push(carry);
        }

        &res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
    }

    #[test]
    fn example2() {
        assert_eq!(vec![4, 3, 2, 2], Solution::plus_one(vec![4, 3, 2, 1]));
    }

    #[test]
    fn example3() {
        assert_eq!(vec![1, 0], Solution::plus_one(vec![9]));
    }
}
