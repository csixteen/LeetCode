pub struct Solution {}

impl Solution {
    pub fn fib(n: i32) -> i32 {
        Self::_fib(n, 0, 1)
    }

    fn _fib(n: i32, a: i32, b: i32) -> i32 {
        if n == 0 {
            a
        } else {
            Self::_fib(n - 1, b, a + b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(1, Solution::fib(2));
    }

    #[test]
    fn test_case2() {
        assert_eq!(2, Solution::fib(3));
    }

    #[test]
    fn test_case3() {
        assert_eq!(3, Solution::fib(4));
    }
}
