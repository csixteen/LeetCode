// https://leetcode.com/problems/destination-city/

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let occur: HashMap<&String, usize> = paths.iter().fold(HashMap::new(), |mut acc, path| {
            acc.entry(&path[0]).and_modify(|e| { *e += 1 }).or_insert(1);
            acc.entry(&path[1]).or_insert(0);
            acc
        });

        occur.iter().find(|&(_, v)| *v == 0).unwrap().0.to_string()
    }

    pub fn dest_city2(paths: Vec<Vec<String>>) -> String {
        let srcs: HashSet<&String> = paths.iter().map(|p| &p[0]).collect();
        let dsts: HashSet<&String> = paths.iter().map(|p| &p[1]).collect();

        dsts.difference(&srcs).next().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_dest_city() {
        assert_eq!(
            "Sao Paulo".to_string(),
            Solution::dest_city(
                vec![
                    vec!["London".to_string(), "New York".to_string()],
                    vec!["New York".to_string(), "Lima".to_string()],
                    vec!["Lima".to_string(), "Sao Paulo".to_string()]
                ]
            )
        );

        assert_eq!(
            "A".to_string(),
            Solution::dest_city(
                vec![
                    vec!["B".to_string(), "C".to_string()],
                    vec!["D".to_string(), "B".to_string()],
                    vec!["C".to_string(), "A".to_string()]
                ]
            )
        );

        assert_eq!(
            "Z".to_string(),
            Solution::dest_city(vec![vec!["A".to_string(), "Z".to_string()]])
        );
    }

    #[test]
    fn test_dest_city2() {
        assert_eq!(
            "Sao Paulo".to_string(),
            Solution::dest_city2(
                vec![
                    vec!["London".to_string(), "New York".to_string()],
                    vec!["New York".to_string(), "Lima".to_string()],
                    vec!["Lima".to_string(), "Sao Paulo".to_string()]
                ]
            )
        );

        assert_eq!(
            "A".to_string(),
            Solution::dest_city2(
                vec![
                    vec!["B".to_string(), "C".to_string()],
                    vec!["D".to_string(), "B".to_string()],
                    vec!["C".to_string(), "A".to_string()]
                ]
            )
        );

        assert_eq!(
            "Z".to_string(),
            Solution::dest_city2(vec![vec!["A".to_string(), "Z".to_string()]])
        );
    }
}
