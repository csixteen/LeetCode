// https://leetcode.com/problems/cheapest-flights-within-k-stops/

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

struct Solution {}

type Flights = Vec<Vec<i32>>;

// (next_cost, next_destination, current_steps)
#[derive(Eq)]
struct Flight(i32, i32, i32);

impl Ord for Flight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Flight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other).reverse())
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

const SRC: usize = 0;
const DST: usize = 1;
const COST: usize = 2;

impl Solution {
    pub fn find_cheapest_price(n: i32, flights: Flights, src: i32, dst: i32, k: i32) -> i32 {
        let mut graph: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();

        flights.iter().for_each(|flight| {
            graph
                .entry(flight[SRC])
                .and_modify(|e| e.push((flight[DST], flight[COST])))
                .or_insert(vec![(flight[DST], flight[COST])]);
        });

        let mut heap: BinaryHeap<Flight> = BinaryHeap::new();
        heap.push(Flight(0, src, -1));

        while !heap.is_empty() {
            if let Some(Flight(next_cost, next_dst, curr_steps)) = heap.pop() {
                if curr_steps > k { continue; }
                if next_dst == dst { return next_cost; }

                for (nd, nc) in graph.get(&next_dst).unwrap_or(&Vec::new()).iter() {
                    heap.push(Flight(next_cost + nc, *nd, curr_steps + 1));
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            200,
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                1,
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            500,
            Solution::find_cheapest_price(
                3,
                vec![vec![0, 1, 100], vec![1, 2, 100], vec![0, 2, 500]],
                0,
                2,
                0,
            ),
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            6,
            Solution::find_cheapest_price(
                4,
                vec![vec![0, 1, 1], vec![0, 2, 5], vec![1, 2, 1], vec![2, 3, 1]],
                0,
                3,
                1,
            ),
        );
    }

    #[test]
    fn test_example4() {
        assert_eq!(
            14,
            Solution::find_cheapest_price(
                10,
                vec![
                    vec![3, 4, 4],
                    vec![2, 5, 6],
                    vec![4, 7, 10],
                    vec![9, 6, 5],
                    vec![7, 4, 4],
                    vec![6, 2, 10],
                    vec![6, 8, 6],
                    vec![7, 9, 4],
                    vec![1, 5, 4],
                    vec![1, 0, 4],
                    vec![9, 7, 3],
                    vec![7, 0, 5],
                    vec![6, 5, 8],
                    vec![1, 7, 6],
                    vec![4, 0, 9],
                    vec![5, 9, 1],
                    vec![8, 7, 3],
                    vec![1, 2, 6],
                    vec![4, 1, 5],
                    vec![5, 2, 4],
                    vec![1, 9, 1],
                    vec![7, 8, 10],
                    vec![0, 4, 2],
                    vec![7, 2, 8],
                ],
                6,
                0,
                7,
            ),
        );
    }
}
