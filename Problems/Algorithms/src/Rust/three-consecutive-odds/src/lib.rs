pub struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut counter = 0;
        for i in arr {
            if i % 2 == 0 {
                counter = 0;
            } else {
                counter += 1;
                if counter == 3 {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert!(!Solution::three_consecutive_odds(vec![2, 6, 4, 1]));
    }

    #[test]
    fn test_case2() {
        assert!(Solution::three_consecutive_odds(vec![
            1, 2, 34, 3, 4, 5, 7, 23, 12
        ]));
    }
}
