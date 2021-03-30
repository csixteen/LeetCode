// https://leetcode.com/problems/kth-largest-element-in-a-stream/

#![allow(dead_code)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut k = KthLargest {
            heap: BinaryHeap::with_capacity(k as usize),
        };

        for n in nums.iter() {
            k.add(*n);
        }

        k
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.heap.capacity() {
            self.heap.push(Reverse(val));
        } else if self.heap.peek().unwrap().0 < val {
            self.heap.pop();
            self.heap.push(Reverse(val));
        }

        self.heap.peek().unwrap().0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kth_largest() {
        let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
        
        assert_eq!(4, obj.add(3));
        assert_eq!(5, obj.add(5));
        assert_eq!(5, obj.add(10));
        assert_eq!(8, obj.add(9));
        assert_eq!(8, obj.add(4));
    }
}
