// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/

#![allow(dead_code)]

use std::cmp::Ordering::*;
use std::rc::Rc;
use std::cell::RefCell;

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

struct Solution;

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    fn node_val(node: &Node) -> i32 {
        node.as_ref().unwrap().borrow().val
    }

    pub fn lowest_common_ancestor(root: Node, p: Node, q: Node) -> Node {
        if let Some(node) = root {
            let p_val = Self::node_val(&p);
            let q_val = Self::node_val(&q);
            let v = node.borrow().val;

            match (p_val.cmp(&v), q_val.cmp(&v)) {
                (Greater, Greater) =>
                    Self::lowest_common_ancestor(node.borrow().right.as_ref().cloned(), p, q),
                (Less, Less) =>
                    Self::lowest_common_ancestor(node.borrow().left.as_ref().cloned(), p, q),
                _ => Some(node),
            }

        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
