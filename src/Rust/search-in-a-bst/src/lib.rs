// https://leetcode.com/problems/search-in-a-binary-search-tree/

use std::cmp::Ordering;
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

struct Solution {}

impl Solution {
    pub fn search_bst(root: Node, val: i32) -> Node {
        if let Some(x) = &root {
            let node = x.borrow();

            if node.val > val {
                return Self::search_bst(node.left.clone(), val);
            } else if node.val < val {
                return Self::search_bst(node.right.clone(), val);
            }
        }

        root
    }

    pub fn search_bst_iter(root: Node, val: i32) -> Node {
        let mut root = root;

        while let Some(x) = root {
            let node = x.borrow();

            match val.cmp(&node.val) {
                Ordering::Equal => return Some(x.clone()),
                Ordering::Less => root = x.borrow().left.clone(),
                Ordering::Greater => root = x.borrow().right.clone(),
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            None,
            Solution::search_bst_iter(None, 1),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode{
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            Solution::search_bst_iter(
                Some(Rc::new(RefCell::new(TreeNode{
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode{
                        val: 7, left: None, right: None
                    }))),
                }))),
                2,
            ),
        );
    }
}
