pub struct Solution {}

impl Solution {
    pub fn my_pow2(x: f64, n: i32) -> f64 {
        #[inline]
        fn my_pow_rec(x: f64, n: i64) -> f64 {
            if n == 0 {
                1_f64
            } else if n < 0 {
                1_f64 / my_pow_rec(x, -n)
            } else if n % 2 == 1 {
                x * my_pow_rec(x * x, (n - 1) / 2)
            } else {
                my_pow_rec(x * x, n / 2)
            }
        }

        my_pow_rec(x, n as i64)
    }

    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut res = 1_f64;

        let mut x = x;
        let mut n = n as i64;
        if n < 0 {
            x = 1_f64 / x;
            n = -n;
        }

        while n > 0 {
            if n % 2 == 1 {
                res *= x;
            }

            x *= x;
            n /= 2;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(1024_f64, Solution::my_pow(2_f64, 10_i32));
    }

    #[test]
    fn test_case2() {
        assert_eq!(9.261_f64, Solution::my_pow(2.1_f64, 3));
    }

    #[test]
    fn test_case3() {
        assert_eq!(0.25_f64, Solution::my_pow(2_f64, -2));
    }

    #[test]
    fn test_case4() {
        assert_eq!(1_f64, Solution::my_pow(10_f64, 0));
    }

    #[test]
    fn test_case5() {
        assert_eq!(10_f64, Solution::my_pow(10_f64, 1));
    }
}
