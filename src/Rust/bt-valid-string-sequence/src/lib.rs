// https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/532/week-5/3315/

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

struct Solution {}

impl Solution {
    pub fn is_valid_sequence(root: Option<Rc<RefCell<TreeNode>>>, arr: Vec<i32>) -> bool {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, seq: &Vec<i32>, i: usize) -> bool {
            if let Some(n) = node {
                let v = n.borrow().val;

                if (i == seq.len() - 1) && 
                    v == seq[i] &&
                    n.borrow().left.is_none() &&
                    n.borrow().right.is_none() {
                        return true;
                } else if i < seq.len() && v != seq[i] {
                    return false;
                } else {
                    let cond = i < seq.len() - 1;
                    let left = if n.borrow().left.is_some() && cond {
                        dfs(&n.borrow().left, seq, i+1)
                    } else {
                        false
                    };

                    let right = if n.borrow().right.is_some() && cond {
                        dfs(&n.borrow().right, seq, i+1)
                    } else {
                        false
                    };

                    return left || right;
                }
            } else {
                false
            }
        }

        dfs(&root, &arr, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        arr: Vec<i32>,
        expected: bool,
    }

    #[test]
    fn test_is_valid_sequence() {
        let tree = Some(Rc::new(RefCell::new(
            TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(
                    TreeNode {
                        val: 1,
                        left: Some(Rc::new(RefCell::new(
                            TreeNode {
                                val: 0,
                                left: None,
                                right: Some(Rc::new(RefCell::new(
                                    TreeNode { val: 1, left: None, right: None }))),
                            }))),
                        right: Some(Rc::new(RefCell::new(
                            TreeNode {
                                val: 1,
                                left: Some(Rc::new(RefCell::new(
                                    TreeNode { val: 0, left: None, right: None }))),
                                right: Some(Rc::new(RefCell::new(
                                    TreeNode { val: 0, left: None, right: None }))),
                            }))),
                    }))),
                right: Some(Rc::new(RefCell::new(
                    TreeNode {
                        val: 0,
                        left: Some(Rc::new(RefCell::new(
                            TreeNode{ val: 0, left: None, right: None}))),
                        right: None,
                    }))),
            })));
        let test_cases = [
            TestCase {
                arr: vec![0, 1, 0, 1],
                expected: true,
            },
            TestCase {
                arr: vec![0, 0, 1],
                expected: false,
            },
            TestCase {
                arr: vec![0, 1, 1],
                expected: false,
            },
            TestCase {
                arr: vec![0, 0, 0],
                expected: true,
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::is_valid_sequence(tree.clone(), case.arr.clone()),
            );
        }
    }
}
