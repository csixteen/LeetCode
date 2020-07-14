#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/merge-two-sorted-lists
import unittest

from lists import (
    ListNode,
    to_linked_list
)


class Solution(object):
    def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
        dummy = ListNode(None)
        curr = dummy

        while l1 and l2:
            if l1.val < l2.val:
                curr.next, l1 = l1, l1.next
            else:
                curr.next, l2 = l2, l2.next

            curr = curr.next

        if l1:
            curr.next = l1
        elif l2:
            curr.next = l2

        return dummy.next


class TestSolution(unittest.TestCase):
    def test_mergeTwoLists(self):
        s = Solution()
        l1 = to_linked_list([1, 2, 4])
        l2 = to_linked_list([1, 3, 4])
        l3 = s.mergeTwoLists(l1, l2)
        self.assertTrue(
            l3 == ListNode(
                1,
                next_=ListNode(
                    1,
                    next_=ListNode(
                        2,
                        next_=ListNode(
                            3,
                            next_=ListNode(
                                4,
                                next_=ListNode(4)))))))


if __name__ == '__main__':
    unittest.main()
