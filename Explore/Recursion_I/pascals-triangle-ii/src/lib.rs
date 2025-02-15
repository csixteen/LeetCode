pub struct Solution {}

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        Solution::_get_row(row_index as usize, vec![1])
    }

    fn _get_row(row_index: usize, acc: Vec<i32>) -> Vec<i32> {
        if row_index == 0 {
            acc
        } else {
            let mut res = vec![1; acc.len() + 1];
            let mut i = 1;
            while i < res.len() - 1 {
                res[i] = acc[i - 1] + acc[i];
                i += 1;
            }

            Self::_get_row(row_index - 1, res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(vec![1, 3, 3, 1], Solution::get_row(3));
    }

    #[test]
    fn test_case2() {
        assert_eq!(vec![1], Solution::get_row(0));
    }

    #[test]
    fn test_case3() {
        assert_eq!(vec![1, 1], Solution::get_row(1));
    }
}
