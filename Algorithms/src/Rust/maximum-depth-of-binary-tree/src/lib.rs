// https://leetcode.com/problems/maximum-depth-of-binary-tree/

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
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
    fn _max_depth(node: &Option<Rc<RefCell<TreeNode>>>, acc: i32) -> i32 {
        if let Some(x) = node {
            Self::_max_depth(&x.borrow().left, acc + 1)
                .max(Self::_max_depth(&x.borrow().right, acc + 1))
        } else {
            acc
        }
    }

    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::_max_depth(&root, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            3,
            Solution::max_depth(
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
