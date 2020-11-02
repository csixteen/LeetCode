// https://leetcode.com/problems/merge-two-sorted-lists/

#![allow(dead_code)]

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

type LinkedListNode = Option<Box<ListNode>>;

impl Solution {
    // Recursive
    pub fn merge_two_lists2(l1: LinkedListNode, l2: LinkedListNode) -> LinkedListNode {
        match (l1, l2) {
            (h @ Some(_), None) => h,
            (None, h @ Some(_)) => h,
            (Some(mut a), Some(mut b)) => {
                if a.val < b.val {
                    a.next = Self::merge_two_lists2(a.next, Some(b));
                    return Some(a)
                } else {
                    b.next = Self::merge_two_lists2(Some(a), b.next);
                    return Some(b)
                }
            },
            _ => None,
        }
    }

    pub fn merge_two_lists(l1: LinkedListNode, l2: LinkedListNode) -> LinkedListNode {
        let (mut l1, mut l2) = (l1, l2);
        let mut dummy = Box::new(ListNode::new(-1));
        let mut curr = &mut dummy;

        while l1.is_some() && l2.is_some() {
            let a = l1.take();
            let b = l2.take();

            if let (Some(mut a_head), Some(mut b_head)) = (a, b) {
                if a_head.val < b_head.val {
                    l1 = a_head.next.take();
                    l2 = Some(b_head);
                    curr = curr.next.get_or_insert(a_head);
                } else {
                    l2 = b_head.next.take();
                    l1 = Some(a_head);
                    curr = curr.next.get_or_insert(b_head);
                }
            }
        }

        if l1.is_some() {
            curr.next = l1;
        } else if l2.is_some() {
            curr.next = l2;
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode::new(4))),
                            })),
                        })),
                    })),
                })),
            })),
            Solution::merge_two_lists(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
            ),
        );
    }

    #[test]
    fn test_example1_recursive() {
        assert_eq!(
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode::new(4))),
                            })),
                        })),
                    })),
                })),
            })),
            Solution::merge_two_lists2(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(4))),
                    })),
                })),
            ),
        );
    }
}
