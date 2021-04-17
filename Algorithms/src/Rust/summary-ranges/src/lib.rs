// https://leetcode.com/problems/summary-ranges/

struct Solution;

impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        fn make_range(a: i32, b: i32) -> String {
            if a == b { format!("{}", a) } else { format!("{}->{}", a, b) }
        }

        fn go(a: &Vec<i32>, l: usize, r: usize, acc: Vec<String>) -> Vec<String> {
            if r >= a.len() { return acc }
            if (r < a.len() - 1) && (a[r] + 1 == a[r+1]) {
                go(a, l, r+1, acc)
            } else {
                go(a, r+1, r+1, [acc, vec![make_range(a[l], a[r])]].concat())
            }
        }

        go(&nums, 0, 0, Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summary_ranges() {
        assert_eq!(
            vec![String::from("0->2"), String::from("4->5"), String::from("7")],
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7])
        );

        assert_eq!(
            vec![
                String::from("0"),
                String::from("2->4"),
                String::from("6"),
                String::from("8->9")
            ],
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9])
        );

        assert_eq!(
            Vec::<String>::new(),
            Solution::summary_ranges(vec![])
        );

        assert_eq!(
            vec![String::from("-1")],
            Solution::summary_ranges(vec![-1])
        );

        assert_eq!(
            vec![String::from("-2147483648->-2147483647"), String::from("2147483647")],
            Solution::summary_ranges(vec![-2147483648, -2147483647, 2147483647])
        );
    }
}
