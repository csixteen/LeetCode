// https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/

use std::rc::Rc;
use std::cell::RefCell;


type BSTNode = Option<Rc<RefCell<TreeNode>>>;


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: BSTNode,
    pub right: BSTNode,
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
    fn rec(preorder: &[i32]) -> BSTNode {
        if let Some(first) = preorder.first() {
            let root = Rc::new(RefCell::new(TreeNode::new(*first)));
            let i = (1..preorder.len())
                .find(|&i| preorder[i] > preorder[0])
                .unwrap_or_else(|| preorder.len());

            root.borrow_mut().left = Self::rec(&preorder[1..i]);
            root.borrow_mut().right = Self::rec(&preorder[i..]);

            Some(root)
        } else {
            None
        }
    }

    pub fn bst_from_preorder(preorder: Vec<i32>) -> BSTNode {
        Self::rec(&preorder)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        assert_eq!(
            None,
            Solution::bst_from_preorder(vec![]),
        );
    }

    #[test]
    fn test_example1() {
        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            Solution::bst_from_preorder(vec![3, 1, 5]),
        );
    }
}
