// https://leetcode.com/problems/closest-binary-search-tree-value/

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

impl Solution {
    pub fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f64) -> i32 {
        if let Some(node) = root {
            let mut vals = vec![node.borrow().val];

            if let Some(l) = &node.borrow().left {
                vals.push(Self::closest_value(Some(l.clone()), target));
            }

            if let Some(r) = &node.borrow().right {
                vals.push(Self::closest_value(Some(r.clone()), target));
            }

            vals.iter().skip(1).fold(vals[0], |acc, i| {
                if ((*i as f64) - target).abs() < ((acc as f64) - target).abs() {
                    *i
                } else {
                    acc
                }
            })
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest_value() {
        assert_eq!(
            4,
            Solution::closest_value(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                }))),
                3.714286
            )
        );

        assert_eq!(
            1,
            Solution::closest_value(
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                4.428571
            )
        );
    }
}
