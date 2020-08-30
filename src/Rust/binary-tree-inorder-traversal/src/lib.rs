// https://leetcode.com/problems/binary-tree-inorder-traversal/submissions/

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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() { return Vec::new() }

        let mut res = Vec::new();
        let mut stack = Vec::new();

        stack.push((root.unwrap(), false));

        while let Some((node, seen)) = stack.pop() {
            if !seen {
                stack.push((node.clone(), true));
                if let Some(left) = &node.borrow().left {
                    stack.push((left.clone(), false));
                }
            } else {
                res.push(node.borrow().val);
                if let Some(right) = &node.borrow().right {
                    stack.push((right.clone(), false));
                }
            }
        }

        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![1, 3, 2],
            Solution::inorder_traversal(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                        right: None,
                    }))),
                }))),
            ),
        );
    }
}
