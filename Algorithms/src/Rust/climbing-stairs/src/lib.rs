// https://leetcode.com/problems/climbing-stairs/

#![allow(dead_code)]
#![allow(unused_assignments)]

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let (mut a, mut b, mut c) = (0, 0, 1);

        for _ in 0..n {
            a = b + c;
            b = c;
            c = a;
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(2, Solution::climb_stairs(2));
    }

    #[test]
    fn test_example2() {
        assert_eq!(3, Solution::climb_stairs(3));
    }
}
