// https://leetcode.com/problems/invert-binary-tree/

#![allow(dead_code)]

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

type BinaryTreeNode = Option<Rc<RefCell<TreeNode>>>;

struct Solution {}

impl Solution {
    pub fn invert_tree(root: BinaryTreeNode) -> BinaryTreeNode {
        match root {
            None => None,
            Some(node) => {
                let left = Self::invert_tree(node.borrow_mut().left.clone());
                let right = Self::invert_tree(node.borrow_mut().right.clone());
                node.borrow_mut().right = left;
                node.borrow_mut().left = right;

                Some(node)
            },
        }
    }

    pub fn invert_tree2(mut root: BinaryTreeNode) -> BinaryTreeNode {
        fn helper(node: &mut BinaryTreeNode) -> BinaryTreeNode {
            node.take().map(|n| {
                let left = helper(&mut n.borrow_mut().left);
                let right = helper(&mut n.borrow_mut().right);

                n.borrow_mut().left = right;
                n.borrow_mut().right = left;

                n
            })
        }

        helper(&mut root)
    }

    pub fn invert_tree3(root: BinaryTreeNode) -> BinaryTreeNode {
        if let Some(node) = root {
            let left = Self::invert_tree3(node.borrow_mut().left.clone());
            let right = Self::invert_tree3(node.borrow_mut().right.clone());

            node.borrow_mut().left = right;
            node.borrow_mut().right = left;

            Some(node)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            Solution::invert_tree3(
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
            }))),
            Solution::invert_tree3(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 7,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                    }))),
                }))),
            ),
        );
    }
}
