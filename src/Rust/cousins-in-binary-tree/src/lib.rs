// https://leetcode.com/problems/cousins-in-binary-tree/

use std::cell::RefCell;
use std::rc::Rc;


pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val: val,
            left: None,
            right: None,
        }
    }
}

struct Solution {}

impl Solution {
    // Returns (parent, height)
    fn dfs(
        node: &Option<Rc<RefCell<TreeNode>>>,
        target: i32,
        depth: i32,
        parent: i32
    ) -> Option<(i32, i32)> {
        if let Some(x) = node {
            let val = x.borrow().val;

            if val == target {
                return Some((parent, depth));
            }

            let left = Self::dfs(&x.borrow().left, target, depth+1, val);
            let right = Self::dfs(&x.borrow().right, target, depth+1, val);

            left.or(right)

        } else {
            None
        }
    }

    pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
        let xx = Self::dfs(&root, x, 0, -1);
        let yy = Self::dfs(&root, y, 0, -1);

        match xx.is_some() && yy.is_some() {
            false => { return false; },
            true => {
                let xx = xx.unwrap();
                let yy = yy.unwrap();

                return (xx.0 != yy.0) && (xx.1 == yy.1);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        root: Option<Rc<RefCell<TreeNode>>>,
        x: i32,
        y: i32,
        expected: bool,
    }

    #[test]
    fn test_is_cousins() {
        let test_cases = [
            TestCase {
                root: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
                x: 4,
                y: 3,
                expected: false,
            },
            TestCase {
                root: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    }))),
                }))),
                x: 5,
                y: 4,
                expected: true,
            },
            TestCase {
                root: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
                x: 2,
                y: 3,
                expected: false,
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::is_cousins(case.root.clone(), case.x, case.y),
            );
        }
    }
}
