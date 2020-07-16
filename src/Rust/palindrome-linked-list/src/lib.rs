// https://leetcode.com/problems/palindrome-linked-list/

#![allow(dead_code)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
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
    pub fn is_palindrome2(head: Option<Box<ListNode>>) -> bool {
        let mut nums = Vec::new();
        let mut curr = &head;

        while let Some(node) = curr {
            nums.push(node.val);
            curr = &node.next;
        }

        nums.iter().eq(nums.iter().rev())
    }

    pub fn is_palindrome(mut head: Option<Box<ListNode>>) -> bool {
        let mut curr = &head;
        let mut len: usize = 0;

        // First, we calculate the length of the list
        while let Some(node) = curr {
            curr = &node.next;
            len += 1;
        }

        if len < 2 { return true; }

        let mid = len / 2;

        // Then we move the `right` pointer to the beginning of the
        // second half of the list. The beginning of the second half
        // will depend on whether the length of the list is odd or even.
        let mut right = &mut head;
        for _ in 0..(mid + (len % 2)) {
            right = &mut right.as_mut().unwrap().next;
        }

        let right = &mut right.take();

        // Now we reverse the first half of the list
        let mut left = head;
        let mut prev = None;
        let mut i = 0;

        while let Some(mut node) = left.take() {
            let future = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            left = future;

            i += 1;
            if i == mid { break; }
        }

        prev == *right
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert!(
            !Solution::is_palindrome(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode::new(2))),
                })),
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert!(
            Solution::is_palindrome(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode::new(1))),
                        })),
                    })),
                })),
            ),
        );
    }
}
