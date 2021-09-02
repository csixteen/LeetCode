// https://leetcode.com/problems/partition-list/

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
    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut lt = Some(Box::new(ListNode::new(-1000)));
        let mut ge = Some(Box::new(ListNode::new(-1000)));
        let mut lt_curr = lt.as_mut();
        let mut ge_curr = ge.as_mut();
        let mut curr = head;

        while let Some(n) = curr {
            if n.val < x {
                if let Some(nn) = lt_curr {
                    nn.next = Some(n.clone());
                    lt_curr = nn.next.as_mut();
                }
            } else {
                if let Some(nn) = ge_curr {
                    nn.next = Some(n.clone());
                    ge_curr = nn.next.as_mut();
                }
            }

            curr = n.next;
        }

        ge_curr.unwrap().next = None;
        lt_curr.unwrap().next = ge.unwrap().next;
        lt.unwrap().next
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
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 3,
                                next: Some(Box::new(ListNode::new(5))),
                            })),
                        })),
                    })),
                })),
            })),
            Solution::partition(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 2,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode::new(2))),
                                })),
                            })),
                        })),
                    })),
                })),
                3
            ),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Some(Box::new(ListNode{
                val: 1,
                next: Some(Box::new(ListNode::new(2))),
            })),
            Solution::partition(
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(1))),
                })),
                2,
            ),
        );
    }
}
