// https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/

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

type BSTNode = Option<Rc<RefCell<TreeNode>>>;

struct Solution;

impl Solution {
    fn rec(nums: &[i32]) -> BSTNode {
        if nums.is_empty() { return None; }

        let mid = nums.len() / 2;

        Some(Rc::new(RefCell::new(TreeNode {
            val: nums[mid],
            left: Self::rec(&nums[0..mid]),
            right: Self::rec(&nums[mid+1..]),
        })))
    }

    pub fn sorted_array_to_bst(nums: Vec<i32>) -> BSTNode {
        Self::rec(&nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(-10)))),
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: None,
                }))),
            }))),
            Solution::sorted_array_to_bst(vec![-10, -3, 0, 5, 9]),
        );
    }
}
