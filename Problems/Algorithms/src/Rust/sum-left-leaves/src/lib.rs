// https://leetcode.com/problems/sum-of-left-leaves/

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
    pub fn sum_of_left_leaves(root: Node) -> i32 {
        fn go(node: Node, left: bool) -> i32 {
            if let Some(n) = node {
                let node = n.borrow();
                match (node.left.as_ref(), node.right.as_ref()) {
                    (None, None) if left => node.val,
                    (x, y) => go(x.map(|c| c.clone()), true) + go(y.map(|c| c.clone()), false),
                }
            } else {
                0
            }
        }

        go(root, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_left_leaves() {
        assert_eq!(
            24,
            Solution::sum_of_left_leaves(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 20,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
                    })))
                })))
            )
        );

        assert_eq!(
            0,
            Solution::sum_of_left_leaves(Some(Rc::new(RefCell::new(TreeNode::new(1)))))
        );
    }
}
