// https://leetcode.com/problems/shortest-word-distance/

struct Solution;

impl Solution {
    pub fn shortest_distance(words_dict: Vec<String>, word1: String, word2: String) -> i32 {
        fn go(dict: &Vec<String>, w1: &String, w2: &String, i1: i32, i2: i32, i: usize, d: i32) -> i32 {
            if i >= dict.len() { return d; }
            let new_i1 = if dict[i] == *w1 { i as i32 } else { i1 };
            let new_i2 = if dict[i] == *w2 { i as i32 } else { i2 };
            let new_d = if new_i1 != -1 && new_i2 != -1 {
                d.min((new_i1 - new_i2).abs())
            } else {
                d
            };

            go(dict, w1, w2, new_i1, new_i2, i+1, new_d)
        }

        go(&words_dict, &word1, &word2, -1, -1, 0, words_dict.len() as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_distance() {
        assert_eq!(
            3,
            Solution::shortest_distance(
                vec![
                    String::from("practice"),
                    String::from("makes"),
                    String::from("perfect"),
                    String::from("coding"),
                    String::from("makes")
                ],
                String::from("coding"),
                String::from("practice")
            )
        );
        assert_eq!(
            1,
            Solution::shortest_distance(
                vec![
                    String::from("practice"),
                    String::from("makes"),
                    String::from("perfect"),
                    String::from("coding"),
                    String::from("makes")
                ],
                String::from("makes"),
                String::from("coding")
            )
        );

        assert_eq!(
            1,
            Solution::shortest_distance(
                vec![
                    "a".to_string(),
                    "c".to_string(),
                    "b".to_string(),
                    "b".to_string(),
                    "a".to_string()
                ],
                "a".to_string(),
                "b".to_string()
            )
        );
    }
}
