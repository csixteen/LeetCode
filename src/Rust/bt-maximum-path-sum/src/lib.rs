// https://leetcode.com/problems/binary-tree-maximum-path-sum/

#![allow(dead_code)]

use std::cmp::max;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, acc: &mut i32) -> i32 {
            if let Some(x) = node {
                let v = x.borrow().val;

                let left = max(dfs(&x.borrow().left, acc), 0);
                let right = max(dfs(&x.borrow().right, acc), 0);

                *acc = max(*acc, v + left + right);

                v + max(left, right)
            } else {
                0
            }
        }

        let mut max_sum = std::i32::MIN;
        dfs(&root, &mut max_sum);

        max_sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        root: Option<Rc<RefCell<TreeNode>>>,
        expected: i32,
    }

    #[test]
    fn test_max_path_sum() {
        let test_cases = [
            TestCase {
                root: Some(
                    Rc::new(
                        RefCell::new(
                            TreeNode {
                                val: -2,
                                left: Some(Rc::new(RefCell::new(
                                    TreeNode { val: -1, left: None, right: None }))),
                                right: None,
                            }))),
                expected: -1,
            },
            TestCase {
                root: Some(Rc::new(RefCell::new(
                                  TreeNode { val: -3, left: None, right: None }))),
                expected: -3
            },
            TestCase {
                root: Some(
                    Rc::new(RefCell::new(
                        TreeNode {
                            val: 1,
                            left: Some(Rc::new(RefCell::new(
                                    TreeNode { val: 2, left: None, right: None }))),
                            right: Some(Rc::new(RefCell::new(
                                    TreeNode { val: 3, left: None, right: None }))),
                        }))),
                expected: 6,
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(case.expected, Solution::max_path_sum(case.root.clone()));
        }
    }
}
