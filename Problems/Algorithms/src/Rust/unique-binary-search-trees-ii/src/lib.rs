// https://leetcode.com/problems/unique-binary-search-trees-ii/

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

struct Solution;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        fn gen_trees(i: i32, j: i32) -> Vec<Node> {
            match i.cmp(&j) {
                Ordering::Greater => vec![None],
                Ordering::Equal => vec![Some(Rc::new(RefCell::new(TreeNode::new(i))))],
                Ordering::Less =>
                    ((i as usize)..=(j as usize)).flat_map(|k| {
                        let left = gen_trees(i, (k-1) as i32);
                        let right = gen_trees((k+1) as i32, j);
                        left.iter().flat_map(|l| {
                            right
                                .iter()
                                .map(|r| Some(Rc::new(RefCell::new(TreeNode {
                                    val: k as i32,
                                    left: l.clone(),
                                    right: r.clone(),
                                })))).collect::<Vec<Node>>()
                        }).collect::<Vec<Node>>()
                    }).collect()
            }
        }

        gen_trees(1, n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            vec![
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    }))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                        right: None,
                    }))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    }))),
                    right: None,
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: None,
                    }))),
                    right: None,
                }))),
            ],
            Solution::generate_trees(3),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            vec![
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            ],
            Solution::generate_trees(1),
        );
    }
}
