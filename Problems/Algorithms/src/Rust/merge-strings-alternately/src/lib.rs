pub struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::with_capacity(word1.len() + word2.len());
        let mut chars1 = word1.chars();
        let mut chars2 = word2.chars();

        loop {
            let (c1, c2) = (chars1.next(), chars2.next());
            if let Some(c) = c1 {
                res.push(c);
            }

            if let Some(c) = c2 {
                res.push(c);
            }

            if c1.is_none() && c2.is_none() {
                break;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(
            "apbqcr",
            &Solution::merge_alternately("abc".into(), "pqr".into())
        );
    }

    #[test]
    fn test_case2() {
        assert_eq!(
            "apbqrs",
            &Solution::merge_alternately("ab".into(), "pqrs".into())
        );
    }

    #[test]
    fn test_case3() {
        assert_eq!(
            "apbqcdef",
            &Solution::merge_alternately("abcdef".into(), "pq".into())
        );
    }

    #[test]
    fn test_case4() {
        assert_eq!(
            "cadf",
            &Solution::merge_alternately("cdf".into(), "a".into())
        );
    }

    #[test]
    fn test_case5() {
        assert_eq!(
            "dbeefebda",
            &Solution::merge_alternately("def".into(), "beebda".into())
        );
    }
}
