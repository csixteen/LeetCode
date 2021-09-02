// https://leetcode.com/problems/split-a-string-in-balanced-strings/

struct Solution;

impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        s.chars().fold((0, 0), |(counter, acc), c| {
            let counter = counter + if c == 'L' { 1 } else { -1 };
            let acc = acc + if counter == 0 { 1 } else { 0 };
            (counter, acc)
        }).1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(4, Solution::balanced_string_split("RLRRLLRLRL".to_string()));
    }

    #[test]
    fn example2() {
        assert_eq!(3, Solution::balanced_string_split("RLLLLRRRLR".to_string()));
    }

    #[test]
    fn example3() {
        assert_eq!(1, Solution::balanced_string_split("LLLLRRRR".to_string()));
    }

    #[test]
    fn example4() {
        assert_eq!(2, Solution::balanced_string_split("RLRRRLLRLL".to_string()));
    }
}
