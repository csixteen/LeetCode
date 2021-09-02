// https://leetcode.com/problems/maximum-number-of-balloons/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let b: HashMap<char, i32> =
            [('b', 1), ('a', 1), ('l', 2), ('o', 2), ('n', 1)]
                .iter().cloned().collect();

        let count = text.chars().fold(HashMap::new(), |mut acc, c| {
            if b.contains_key(&c) {
                acc.entry(c).and_modify(|e| { *e += 1 }).or_insert(1);
            }
            acc
        });

        b.iter().map(|(k, v)| count.get(k).map_or(0, |v2| v2 / v)).min().unwrap()
    }

    pub fn max_number_of_balloons2(text: String) -> i32 {
        let mut count = [0_i32; 5];

        text.chars().for_each(|c| {
            match c {
                'b' => count[0] += 2,
                'a' => count[1] += 2,
                'l' => count[2] += 1,
                'o' => count[3] += 1,
                'n' => count[4] += 2,
                _ => (),
            }
        });

        count.iter().min().unwrap() / 2
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_max_number_of_balloons() {
        assert_eq!(1, Solution::max_number_of_balloons("nlaebolko".to_string()));
        assert_eq!(2, Solution::max_number_of_balloons("loonbalxballpoon".to_string()));
        assert_eq!(0, Solution::max_number_of_balloons("leetcode".to_string()));
    }

    #[test]
    fn test_max_number_of_balloons2() {
        assert_eq!(1, Solution::max_number_of_balloons2("nlaebolko".to_string()));
        assert_eq!(2, Solution::max_number_of_balloons2("loonbalxballpoon".to_string()));
        assert_eq!(0, Solution::max_number_of_balloons2("leetcode".to_string()));
    }
}
