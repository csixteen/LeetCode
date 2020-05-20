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

struct Solution {}

impl Solution {
    fn inorder(node: &Option<Rc<RefCell<TreeNode>>>, k: i32, acc: &mut Vec<i32>) {
        if let Some(x) = node {
            Self::inorder(&x.borrow().left, k, acc);
            acc.push(x.borrow().val);
            Self::inorder(&x.borrow().right, k, acc);
        }
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut acc = Vec::new();

        Self::inorder(&root, k, &mut acc);

        acc[(k - 1) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
        })));

        assert_eq!(1, Solution::kth_smallest(tree, 1));
    }

    #[test]
    fn test_example2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: None
                }))),
                right:Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
        })));

        assert_eq!(3, Solution::kth_smallest(tree, 3));
    }
}
