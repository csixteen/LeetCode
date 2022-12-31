// https://leetcode.com/problems/average-of-levels-in-binary-tree/

#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

type Node = Option<Rc<RefCell<TreeNode>>>;

struct Solution;

impl Solution {
    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut res = Vec::new();

        let mut queue = VecDeque::new();
        queue.push_front(root.unwrap());

        while !queue.is_empty() {
            let count = queue.len();
            let mut acc = 0_f64;
            for _ in 0..count {
                let node = queue.pop_back().unwrap();
                acc += node.borrow().val as f64;

                if let Some(left) = &node.borrow().left {
                    queue.push_front(left.clone());
                };

                if let Some(right) = &node.borrow().right {
                    queue.push_front(right.clone());
                };
            }

            res.push(acc / count as f64);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
