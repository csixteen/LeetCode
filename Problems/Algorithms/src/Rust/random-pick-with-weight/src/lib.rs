// https://leetcode.com/problems/random-pick-with-weight/

#![allow(dead_code)]

use std::collections::BTreeMap;

extern "C" {
    fn rand() -> u32;
}

struct Solution {
    prefix_sums: BTreeMap<u32, i32>,
    total_sum: u32,
}

impl Solution {
    fn rand(&self) -> u32 {
        unsafe {
            rand() % self.total_sum + 1
        }
    }

    fn new(w: Vec<i32>) -> Self {
        let mut prefix_sums: BTreeMap<u32, i32> = BTreeMap::new();
        let mut total_sum: u32 = 0;

        for (i, &weight) in w.iter().enumerate() {
            total_sum += weight as u32;
            prefix_sums.insert(total_sum, i as i32);
        }

        Solution { prefix_sums, total_sum }
    }

    fn pick_index(&self) -> i32 {
        *self.prefix_sums
            .range(self.rand()..)
            .next()
            .unwrap().1
    }
}
