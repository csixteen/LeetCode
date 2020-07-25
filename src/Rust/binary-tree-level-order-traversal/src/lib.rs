// https://leetcode.com/problems/binary-tree-level-order-traversal/

#![allow(dead_code)]

use std::cell::RefCell;
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

struct Solution;

impl Solution {
    fn _level_order(node: &Option<Rc<RefCell<TreeNode>>>, level: usize, acc: &mut Vec<Vec<i32>>) {
        if let Some(n) = node {
            if acc.len() == level {
                acc.push(Vec::new());
            }

            acc[level].push(n.borrow().val);

            Self::_level_order(&n.borrow().left, level + 1, acc);
            Self::_level_order(&n.borrow().right, level + 1, acc);
        }
    }

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut acc: Vec<Vec<i32>> = Vec::new();

        Self::_level_order(&root, 0, &mut acc);

        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![vec![3], vec![9, 20], vec![15, 7]],
            Solution::level_order(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 9,
                        left: None,
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 20,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    }))),
                }))),
            ),
        );
    }
}
