pub struct Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        if x < 2 {
            return x;
        }

        let x = x as i64;
        let mut left = 2;
        let mut right = x / 2;

        while left <= right {
            let root = left + (right - left) / 2;
            let res = root * root;
            if res == x {
                return root as i32;
            } else if res > x {
                right = root - 1;
            } else {
                left = root + 1;
            }
        }

        right as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2, Solution::my_sqrt(4));
    }

    #[test]
    fn test2() {
        assert_eq!(2, Solution::my_sqrt(8));
    }

    #[test]
    fn test3() {
        assert_eq!(3, Solution::my_sqrt(9));
    }

    #[test]
    fn test4() {
        assert_eq!(3, Solution::my_sqrt(10));
    }

    #[test]
    fn test5() {
        assert_eq!(4, Solution::my_sqrt(16));
    }
}
