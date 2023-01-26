// https://leetcode.com/problems/minimum-absolute-difference-in-bst/

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

type Node = Option<Rc<RefCell<TreeNode>>>;

struct Solution;

impl Solution {
    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn go(node: &Node, prev: i32, acc: i32) -> (i32, i32) {
            if let Some(n) = node {
                let (p, mut a) = go(&n.borrow().left, prev, acc);
                a = a.min(n.borrow().val.saturating_sub(p));
                go(&n.borrow().right, n.borrow().val, a)
            } else {
                (prev, acc)
            }
        }

        go(&root, i32::MIN, i32::MAX).1
    }

    pub fn get_minimum_difference2(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn go(node: &Node, acc: &mut Vec<i32>) {
            if let Some(n) = node {
                let b = n.borrow();

                if let Some(_) = &b.left {
                    go(&b.left, acc);
                }

                acc.push(b.val);

                if let Some(_) = &b.right {
                    go(&b.right, acc);
                }
            }
        }

        let mut acc = Vec::new();
        go(&root, &mut acc);
        (0..acc.len()-1).fold(i32::MAX, |diff, i| diff.min((acc[i] - acc[i+1]).abs()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            1,
            Solution::get_minimum_difference(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(6))))
                })))
            )
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            1,
            Solution::get_minimum_difference(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 48,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(12)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(49))))
                    }))),
                })))
            )
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            9,
            Solution::get_minimum_difference(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 236,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 104,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(227))))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 701,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(911))))
                    })))
                })))
            )
        );
    }
}
