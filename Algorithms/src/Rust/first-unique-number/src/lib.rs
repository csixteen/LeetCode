// https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/531/week-4/3313/

#![allow(dead_code)]

use std::collections::HashMap;

struct FirstUnique {
    queue: Vec<i32>,
    count: HashMap<i32, i32>,
    first_index: Option<usize>,
}

impl FirstUnique {
    fn new(nums: Vec<i32>) -> Self {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in &nums {
            map
                .entry(*i)
                .and_modify(|e| { *e += 1 })
                .or_insert(1);
        }

        let mut i = 0;
        while i < nums.len() && map.get(&nums[i]).unwrap() > &1 {
            i += 1;
        }

        let first_index = if i == nums.len() { None } else { Some(i) };

        FirstUnique {
            queue: nums,
            count: map,
            first_index: first_index,
        }
    }

    fn show_first_unique(&self) -> i32 {
        match self.first_index {
            Some(v) => self.queue[v],
            None => -1,
        }
    }

    fn add(&mut self, value: i32) {
        self.queue.push(value);
        self.count.entry(value)
            .and_modify(|e| { *e += 1 })
            .or_insert(1);

        match self.first_index {
            Some(v) => if self.count.get(&self.queue[v]).unwrap() > &1 {
                let mut i = v;
                while i < self.queue.len() && self.count.get(&self.queue[i]).unwrap() > &1 {
                    i += 1;
                }

                self.first_index = if i == self.queue.len() { None } else { Some(i) };
            },
            None => if self.count.get(&value).unwrap() == &1 {
                self.first_index = Some(self.queue.len() - 1);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_unique1() {
        let mut fu = FirstUnique::new(vec![2, 3, 5]);

        assert_eq!(2, fu.show_first_unique());
        fu.add(5);
        assert_eq!(2, fu.show_first_unique());
        fu.add(2);
        assert_eq!(3, fu.show_first_unique());
        fu.add(3);
        assert_eq!(-1, fu.show_first_unique());
    }

    #[test]
    fn test_first_unique2() {
        let mut fu = FirstUnique::new(vec![7, 7, 7, 7, 7, 7]);

        assert_eq!(-1, fu.show_first_unique());
        fu.add(7);
        fu.add(3);
        fu.add(3);
        fu.add(7);
        fu.add(17);
        assert_eq!(17, fu.show_first_unique());
    }

    #[test]
    fn test_first_unique_3() {
        let mut fu = FirstUnique::new(vec![809]);

        assert_eq!(809, fu.show_first_unique());
        fu.add(809);
        assert_eq!(-1, fu.show_first_unique());
    }
}
