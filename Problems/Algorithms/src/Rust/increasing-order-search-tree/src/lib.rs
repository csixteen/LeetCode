// https://leetcode.com/problems/increasing-order-search-tree/

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

struct Solution;

impl Solution {
    pub fn increasing_bst(root: Node) -> Node {
        fn tree_to_vec(node: &Node, acc: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();

                if let Some(_) = &n.left {
                    tree_to_vec(&n.left, acc);
                }

                acc.push(n.val);

                if let Some(_) = &n.right {
                    tree_to_vec(&n.right, acc);
                }
            }
        }

        fn vec_to_tree(vs: &Vec<i32>, i: usize) -> Node {
            if i >= vs.len() {
                None
            } else {
                let node = TreeNode {
                    val: vs[i],
                    left: None,
                    right: vec_to_tree(vs, i+1)
                };

                Some(Rc::new(RefCell::new(node)))
            }
        }

        let mut acc = Vec::new();
        tree_to_vec(&root, &mut acc);
        vec_to_tree(&acc, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 5,
                                left: None,
                                right: Some(Rc::new(RefCell::new(TreeNode {
                                    val: 6,
                                    left: None,
                                    right:  Some(Rc::new(RefCell::new(TreeNode {
                                        val: 7,
                                        left: None,
                                        right: Some(Rc::new(RefCell::new(TreeNode {
                                            val: 8,
                                            left: None,
                                            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                                        }))),
                                    }))),
                                }))),
                            }))),
                        }))),
                    }))),
                }))),
            }))),
            Solution::increasing_bst(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 2,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                            right: None,
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 8,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                        }))),
                    }))),
                }))))
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
                })))
            }))),
            Solution::increasing_bst(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
                })))
            )
        );
    }
}
