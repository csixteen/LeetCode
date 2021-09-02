// https://leetcode.com/problems/relative-ranks/

use std::collections::{BinaryHeap, HashMap};

struct Solution;

impl Solution {
    fn index_to_rank(i: usize) -> String {
        match i {
            0 => "Gold Medal".to_string(),
            1 => "Silver Medal".to_string(),
            2 => "Bronze Medal".to_string(),
            _ => (i+1).to_string(),
        }
    }

    pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
        let mut sorted = score.iter().fold(BinaryHeap::new(), |mut acc, s| {
            acc.push(s);
            acc
        });

        let ranks = (0..score.len()).fold(HashMap::new(), |mut acc, i| {
            acc.insert(sorted.pop().unwrap(), Self::index_to_rank(i));
            acc
        });

        score.iter().map(|s| ranks.get(s).unwrap()).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            vec![
                "Gold Medal".to_string(),
                "Silver Medal".to_string(),
                "Bronze Medal".to_string(),
                "4".to_string(),
                "5".to_string(),
            ],
            Solution::find_relative_ranks(vec![5, 4, 3, 2, 1]),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            vec![
                "Gold Medal".to_string(),
                "5".to_string(),
                "Bronze Medal".to_string(),
                "Silver Medal".to_string(),
                "4".to_string(),
            ],
            Solution::find_relative_ranks(vec![10, 3, 8, 9, 4]),
        );
    }
}
