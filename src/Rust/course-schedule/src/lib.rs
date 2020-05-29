// https://leetcode.com/problems/course-schedule/

use std::collections::{HashMap, HashSet, VecDeque};

const TO: usize = 0;
const FROM: usize = 1;

struct Solution {}

impl Solution {
    // Topological sort, Kahn's algorithm
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut from_graph: HashMap<i32, HashSet<i32>> = HashMap::new();
        let mut to_graph: HashMap<i32, HashSet<i32>> = HashMap::new();
        
        for pair in prerequisites.iter() {
            to_graph.entry(pair[TO]).or_insert(HashSet::new()).insert(pair[FROM]);
            from_graph.entry(pair[FROM]).or_insert(HashSet::new()).insert(pair[TO]);
        }

        let mut no_incoming: VecDeque<i32> =
            from_graph.keys().cloned().filter(|k| !to_graph.contains_key(k)).collect();

        while !no_incoming.is_empty() {
            let n = no_incoming.pop_back().unwrap();

            if !from_graph.contains_key(&n) { continue; }

            for m in from_graph.get(&n).unwrap().iter() {
                let mut x = to_graph.get(&m).unwrap().clone();
                x.remove(&n);
                if x.is_empty() { no_incoming.push_front(*m); }
                to_graph.insert(*m, x);
            }
        }

        to_graph.values().all(|x| x.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            true,
            Solution::can_finish(2, vec![vec![1, 0]]),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            false,
            Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            true,
            Solution::can_finish(
                10,
                vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![4, 2]]
            ),
        );
    }

    #[test]
    fn test_example4() {
        assert_eq!(
            false,
            Solution::can_finish(
                5,
                vec![vec![1, 0], vec![2, 1], vec![3, 2], vec![0, 3]],
            ),
        );
    }
}
