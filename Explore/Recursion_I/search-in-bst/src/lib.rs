use std::{cell::RefCell, rc::Rc};

type T = Rc<RefCell<TreeNode>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<T>,
    pub right: Option<T>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn search_bst(root: Option<T>, val: i32) -> Option<T> {
        if let Some(node) = root {
            let x = node.borrow().val;
            if val == x {
                Some(node)
            } else if val < x {
                Self::search_bst(node.borrow().left.as_ref().map(|l| l.clone()), val)
            } else {
                Self::search_bst(node.borrow().right.as_ref().map(|r| r.clone()), val)
            }
        } else {
            None
        }
    }
}
