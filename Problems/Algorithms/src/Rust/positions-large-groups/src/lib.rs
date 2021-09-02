// https://leetcode.com/problems/positions-of-large-groups/

use std::str::Chars;

struct Solution;

impl Solution {
    pub fn large_group_positions(s: String) -> Vec<Vec<i32>> {
        fn go(mut xs: Chars, prev: Option<char>, i: i32, j: i32, mut acc: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            match (xs.next(), prev) {
                (None, _)            => {
                    if j-i >= 3 {
                        acc.push(vec![i, j-1]);
                    }
                    acc
                },
                (Some(c), None)      => go(xs, Some(c), i, j+1, acc),
                (Some(c1), Some(c2)) => {
                    if c1 == c2 {
                        go(xs, Some(c1), i, j+1, acc)
                    } else {
                        if j-i >= 3 {
                            acc.push(vec![i, j-1]);
                        }

                        go(xs, Some(c1), j, j+1, acc)
                    }
                },
            }
        }

        go(s.chars(), None, 0, 0, Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            vec![vec![3, 6]],
            Solution::large_group_positions("abbxxxxzzy".to_string()),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            Solution::large_group_positions("abc".to_string()),
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            vec![vec![3, 5], vec![6, 9], vec![12, 14]],
            Solution::large_group_positions("abcdddeeeeaabbbcd".to_string()),
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            Vec::<Vec<i32>>::new(),
            Solution::large_group_positions("aba".to_string()),
        );
    }
}
