// https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

struct Solution;

impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn go(node: Option<Box<ListNode>>, prev: Option<i32>) -> Option<Box<ListNode>> {
            if let Some(mut n) = node {
                let x = Some(n.val);
                if x == prev {
                    go(n.next, prev)
                } else {
                    match n.next {
                        Some(n2) if n2.val == n.val => go(Some(n2), x),
                        _ => {
                            n.next = go(n.next, x);
                            Some(Box::new(*n))
                        }
                    }
                }
            } else {
                None
            }
        }

        go(head, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(5))),
                })),
            })),
            Solution::delete_duplicates(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 3,
                                next: Some(Box::new(ListNode {
                                    val: 4,
                                    next: Some(Box::new(ListNode {
                                        val: 4,
                                        next: Some(Box::new(ListNode::new(5))),
                                    })),
                                })),
                            })),
                        })),
                    })),
                })),
            ),
        );
    }
}
