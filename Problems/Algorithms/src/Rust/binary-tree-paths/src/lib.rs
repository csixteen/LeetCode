// https://leetcode.com/problems/binary-tree-paths/

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

type Node = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        fn go(node: Node, curr: Vec<i32>, acc: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
            if let Some(n) = node {
                let val = n.borrow().val;
                let curr = [curr, vec![val]].concat();
                let left = &n.borrow().left;
                let right = &n.borrow().right;

                match (left, right) {
                    (None, None) => [acc, vec![curr]].concat(),
                    _ => {
                        let mut l = go(
                            left.as_ref().map(|c| c.clone()),
                            curr.clone(),
                            acc.clone()
                        );
                        l.extend(
                            go(
                                right.as_ref().map(|c| c.clone()),
                                curr,
                                acc
                            )
                        );
                        l
                    }
                }

            } else {
                acc
            }
        }

        go(root, Vec::new(), Vec::new()).iter().map(|v| {
            v.iter().map(|i| i.to_string()).collect::<Vec<String>>().join("->")
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_tree_paths() {
        assert_eq!(
            vec![String::from("1->2->5"), String::from("1->3")],
            Solution::binary_tree_paths(
                Some(Rc::new(RefCell::new(TreeNode{
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode{
                        val:2,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
                })))
            )
        );

        assert_eq!(
            vec![String::from("1")],
            Solution::binary_tree_paths(Some(Rc::new(RefCell::new(TreeNode::new(1)))))
        );
    }
}
