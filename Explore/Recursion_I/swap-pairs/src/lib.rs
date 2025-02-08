use std::borrow::Borrow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }

    pub fn iter(&self) -> ListIterator {
        ListIterator { curr: Some(self) }
    }
}

pub struct ListIterator<'a> {
    curr: Option<&'a ListNode>,
}

impl Iterator for ListIterator<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(curr) = self.curr {
            let res = curr.val;
            self.curr = curr.next.as_ref().map(|next| next.borrow());
            Some(res)
        } else {
            None
        }
    }
}

pub struct Solution {}

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut h) => match h.next {
                Some(mut next) => {
                    h.next = Self::swap_pairs(next.next);
                    next.next = Some(h);
                    Some(next)
                }
                None => Some(h),
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let list = ListNode {
            val: 0,
            next: Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None })),
                })),
            })),
        };

        let xs: Vec<_> = list.iter().collect();
        assert_eq!(xs, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_case1() {
        let list = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            })),
        };
        let head = Solution::swap_pairs(Some(Box::new(list)));
        let xs: Vec<_> = head.unwrap().iter().collect();
        assert_eq!(xs, vec![2, 1, 4, 3]);
    }

    #[test]
    fn test_case2() {
        assert!(Solution::swap_pairs(None).is_none());
    }

    #[test]
    fn test_case3() {
        let list = Some(Box::new(ListNode::new(1)));
        let head = Solution::swap_pairs(list);
        let xs: Vec<_> = head.unwrap().iter().collect();
        assert_eq!(xs, vec![1]);
    }

    #[test]
    fn test_case4() {
        let list = ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode::new(3))),
            })),
        };
        let head = Solution::swap_pairs(Some(Box::new(list)));
        let xs: Vec<_> = head.unwrap().iter().collect();
        assert_eq!(xs, vec![2, 1, 3]);
    }
}
