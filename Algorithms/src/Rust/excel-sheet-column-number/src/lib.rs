// https://leetcode.com/problems/excel-sheet-column-number/

struct Solution;

impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title.bytes().fold(0, |acc, b| {
            acc*26 + ((b - b'A' + 1) as i32)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn it_works() {
        assert_eq!(1, Solution::title_to_number("A".to_string()));
        assert_eq!(28, Solution::title_to_number("AB".to_string()));
        assert_eq!(701, Solution::title_to_number("ZY".to_string()));
        assert_eq!(2147483647, Solution::title_to_number("FXSHRXW".to_string()));
    }
}
