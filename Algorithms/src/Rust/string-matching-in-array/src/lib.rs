// https://leetcode.com/problems/string-matching-in-an-array/

struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut res = Vec::new();
        let mut words = words;
        words.sort_by(|a, b| a.len().cmp(&b.len()));

        for i in 0..words.len()-1 {
            for j in i+1..words.len() {
                if words[j].contains(&words[i]) {
                    res.push(words[i].to_owned());
                    break;
                }
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            vec!["as".to_string(), "hero".to_string()],
            Solution::string_matching(
                vec![
                    "mass".to_string(),
                    "as".to_string(),
                    "hero".to_string(),
                    "superhero".to_string(),
                ],
            ),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            vec!["et".to_string(), "code".to_string()],
            Solution::string_matching(
                vec![
                    "leetcode".to_string(),
                    "et".to_string(),
                    "code".to_string(),
                ],
            ),
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            Vec::<String>::new(),
            Solution::string_matching(
                vec!["blue".to_string(), "green".to_string(), "bu".to_string()],
            ),
        );
    }
}
