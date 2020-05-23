#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/odd-even-linked-list/

from typing import List
import unittest


class ListNode:
    def __init__(self, val: int = 0, _next=None):
        self.val = val
        self.next = _next

    def __repr__(self):
        return f"val: {self.val}, next: ({self.next})"


class Solution:
    def oddEvenList(self, head: ListNode)-> ListNode:
        if head is None or head.next is None:
            return head

        odd, even, even_head = head, head.next, head.next

        while even and even.next:
            odd.next = odd.next.next
            odd = odd.next
            even.next = even.next.next
            even = even.next

        odd.next = even_head

        return head


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.s = Solution()

    def list_equals(self, head: ListNode, a: List) -> bool:
        curr = head

        for item in a:
            if curr is None or curr.val != item:
                return False

            curr = curr.next

        return True

    def test_empty_list(self):
        self.assertTrue(
            self.list_equals(
                self.s.oddEvenList(None),
                [],
            ),
        )

    def test_singleton(self):
        self.assertTrue(
            self.list_equals(
                self.s.oddEvenList(ListNode(val=1)),
                [1],
            ),
        )

    def test_two_elements(self):
        self.assertTrue(
            self.list_equals(
                self.s.oddEvenList(ListNode(val=1, _next=ListNode(val=2))),
                [1, 2],
            ),
        )

    def test_even_number_of_elements(self):
        self.assertTrue(
            self.list_equals(
                self.s.oddEvenList(
                    ListNode(
                        val=1,
                        _next=ListNode(
                            val=2,
                            _next=ListNode(
                                val=3,
                                _next=ListNode(val=4))))),
                [1, 3, 2, 4],
            ),
        )

    def test_odd_number_of_elements(self):
        self.assertTrue(
            self.list_equals(
                self.s.oddEvenList(
                    ListNode(
                        val=1,
                        _next=ListNode(
                            val=2,
                            _next=ListNode(
                                val=3,
                                _next=ListNode(
                                    val=4,
                                    _next=ListNode(val=5)))))),
                [1, 3, 5, 2, 4],
            ),
        )


if __name__ == "__main__":
    unittest.main(verbosity=2)
