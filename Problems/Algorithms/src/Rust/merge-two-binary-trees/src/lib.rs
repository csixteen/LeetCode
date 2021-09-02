// https://leetcode.com/problems/merge-two-binary-trees/

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
    pub fn merge_trees(root1: Node, root2: Node) -> Node {
        match (root1.as_ref(), root2.as_ref()) {
            (None, _) => root2,
            (_, None) => root1,
            (Some(a), Some(b)) => {
                let mut ret = TreeNode::new(a.borrow().val + b.borrow().val);
                ret.left = Self::merge_trees(a.borrow().left.clone(), b.borrow().left.clone());
                ret.right = Self::merge_trees(a.borrow().right.clone(), b.borrow().right.clone());

                Some(Rc::new(RefCell::new(ret)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_trees() {
        assert_eq!(
            Solution::merge_trees(
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: None
                })))
            ),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None
            })))
        );
    }
}
