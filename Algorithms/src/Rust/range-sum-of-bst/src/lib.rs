// https://leetcode.com/problems/range-sum-of-bst/

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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        fn go(node: &Option<Rc<RefCell<TreeNode>>>, lo: i32, hi: i32) -> i32 {
            if let Some(n) = node {
                let mut total = 0;
                let v = n.borrow();

                if v.val >= lo && v.val <= hi {
                    total += v.val;
                }

                total + go(&v.left, lo, hi) + go(&v.right, lo, hi)
            } else {
                0
            }
        }

        go(&root, low, high)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            32,
            Solution::range_sum_bst(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 10,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 15,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(18)))),
                    }))),
                }))),
                7,
                15,
            ),
        );
    }
}
