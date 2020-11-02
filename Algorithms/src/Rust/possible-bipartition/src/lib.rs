// https://leetcode.com/problems/possible-bipartition/

use std::collections::HashMap;

struct Solution {}

impl Solution {
    fn dfs(
        node: i32,
        color: i32,
        color_map: &mut HashMap<i32, i32>,
        graph: &HashMap<i32, Vec<i32>>
    ) -> bool {
        if let Some(&x) = color_map.get(&node) {
            return x == color
        } else {
            color_map.insert(node, color);
            graph
                .get(&node)
                .unwrap()
                .iter()
                .all(|&x| Self::dfs(x, color ^ 1, color_map, &graph))
        }
    }

    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        // Initialize the graph, represented as adjancency lists
        let mut graph = HashMap::new();
        for dis in dislikes.iter() {
            graph.entry(dis[0]).or_insert(Vec::new()).push(dis[1]);
            graph.entry(dis[1]).or_insert(Vec::new()).push(dis[0]);
        }

        let mut color_map: HashMap<i32, i32> = HashMap::new();

        // Check whether we can correctly color each of the connected
        // components of the graph
        graph.keys().all(|&k| match color_map.get(&k) {
            None => Self::dfs(k, 0, &mut color_map, &graph),
            _ => true,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            true,
            Solution::possible_bipartition(
                4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            false,
            Solution::possible_bipartition(
                3, vec![vec![1, 2], vec![1, 3], vec![2, 3]],
            ),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            false,
            Solution::possible_bipartition(
                5,
                vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]],
            ),
        );
    }
}
