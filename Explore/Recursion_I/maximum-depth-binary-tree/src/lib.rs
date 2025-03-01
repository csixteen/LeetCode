use ::std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::_max_depth(&root, 0)
    }

    fn _max_depth(root: &Option<Rc<RefCell<TreeNode>>>, acc: i32) -> i32 {
        if let Some(node) = root {
            let left = Self::_max_depth(&node.borrow().left, acc);
            let right = Self::_max_depth(&node.borrow().right, acc);
            1 + left.max(right)
        } else {
            acc
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        let tree = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        }));

        assert_eq!(3, Solution::max_depth(Some(tree)));
    }

    #[test]
    fn test_case2() {
        let tree = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        }));

        assert_eq!(2, Solution::max_depth(Some(tree)));
    }
}
