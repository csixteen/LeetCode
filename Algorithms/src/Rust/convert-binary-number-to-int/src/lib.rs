// https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/

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
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut ret = 0_i32;
        let mut curr = head;

        while let Some(node) = curr {
            ret = (ret << 1) | node.val;
            curr = node.next;
        }

        ret
    }

    pub fn get_decimal_value2(head: Option<Box<ListNode>>) -> i32 {
        fn go(node: Option<Box<ListNode>>, acc: i32) -> i32 {
            if let Some(n) = node {
                go(n.next, (acc << 1) | n.val)
            } else {
                acc
            }
        }

        go(head, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_decimal_value() {
        assert_eq!(
            5,
            Solution::get_decimal_value(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode::new(1))),
                    }))
                }))
            ),
        );

        assert_eq!(
            0,
            Solution::get_decimal_value(Some(Box::new(ListNode::new(0))))
        );
    }

    #[test]
    fn test_get_decimal_value2() {
        assert_eq!(
            5,
            Solution::get_decimal_value2(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode::new(1))),
                    }))
                }))
            ),
        );

        assert_eq!(
            0,
            Solution::get_decimal_value2(Some(Box::new(ListNode::new(0))))
        );
    }
}
