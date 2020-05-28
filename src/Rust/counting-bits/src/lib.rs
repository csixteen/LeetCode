// https://leetcode.com/problems/counting-bits/

struct Solution {}

impl Solution {
    pub fn count_bits(num: i32) -> Vec<i32> {
        let num = num as usize;

        (1..=num).fold(vec![0; num + 1], |mut acc, x| {
            acc[x] = acc[x & (x-1)] + 1;
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![0, 1, 1],
            Solution::count_bits(2),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            vec![0, 1, 1, 2, 1, 2],
            Solution::count_bits(5),
        );
    }
}
