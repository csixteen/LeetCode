// https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/

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
    fn zigzag(root: &Option<Rc<RefCell<TreeNode>>>, acc: &mut Vec<Vec<i32>>) {
        if root.is_none() { return () }

        let mut stack = Vec::new();
        stack.push((root.clone().unwrap(), 0));

        while let Some((node, level)) = stack.pop() {
            if acc.len() <= level {
                acc.push(Vec::new());
            }

            acc[level].push(node.borrow().val);

            if let Some(right) = &node.borrow().right {
                stack.push((right.clone(), level + 1));
            }

            if let Some(left) = &node.borrow().left {
                stack.push((left.clone(), level + 1));
            }
        }
    }

    fn reverse_odd(res: &mut Vec<Vec<i32>>) {
        for i in (1..res.len()).step_by(2) {
            res[i].reverse();
        }
    }

    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        if root.is_none() { return Vec::new() }

        let mut res = Vec::new();
        Solution::zigzag(&root, &mut res);
        Solution::reverse_odd(&mut res);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![vec![3], vec![20, 9], vec![15, 7]],
            Solution::zigzag_level_order(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
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
