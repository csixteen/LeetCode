#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/reverse-linked-list/

import unittest


class ListNode:
    def __init__(self, x, _next=None):
        self.val = x
        self.next = _next


class Solution:
    def reverseList(self, head: ListNode) -> ListNode:
        if not head or not head.next:
            return head

        prev = None
        while head:
            future = head.next
            head.next = prev
            prev = head
            head = future

        return prev


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5))))),
                ListNode(5, ListNode(4, ListNode(3, ListNode(2, ListNode(1))))),
            ),
            (None, None),
            (ListNode(1), ListNode(1)),
        ]

    def equal_lists(self, a: ListNode, b: ListNode) -> bool:
        if a is None and b is None:
            return True
        elif a is None or b is None:
            return False
        elif a.val != b.val:
            return False

        next_a = a.next
        next_b = b.next

        return self.equal_lists(next_a, next_b)

    def test_reverseList(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertTrue(self.equal_lists(expected, s.reverseList(_input)))


if __name__ == "__main__":
    unittest.main(verbosity=2)
