// https://leetcode.com/problems/group-anagrams/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let groups: HashMap<String, Vec<String>> = strs
            .iter()
            .fold(HashMap::new(), |mut acc, s| {
                let mut pre_sort: Vec<char> = s.clone().chars().collect();
                pre_sort.sort_unstable();
                let post_sort = pre_sort.iter().collect::<String>();
                acc.entry(post_sort).or_insert(Vec::new()).push(s.to_string());
                acc
            });

        groups.values().fold(
            Vec::new(), |mut acc, v| {
                acc.push(v.to_vec());
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(1, 1 + 0);
    }
}
