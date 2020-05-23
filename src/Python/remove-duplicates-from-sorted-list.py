#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/remove-duplicates-from-sorted-list
import unittest

from lists import (
    ListNode,
    to_linked_list,
    to_native
)


class Solution(object):
    def deleteDuplicates(self, head):
        """
        :type head: ListNode
        :rtype: ListNode
        """
        if head is None:
            return None

        new_head = tmp = ListNode(head.val)
        seen = {head.val}
        head = head.next
        while head is not None:
            if head.val not in seen:
                tmp.next = ListNode(head.val)
                tmp = tmp.next
                seen.add(head.val)
            head = head.next

        return new_head


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                to_linked_list([1, 1, 2, 3, 3]),
                [1, 2, 3]
            ),
            (
                to_linked_list([1, 1, 2]),
                [1, 2]
            ),
            (
                to_linked_list([1]),
                [1]
            )
        ]

    def test_deleteDuplicates(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(to_native(s.deleteDuplicates(i)), e)


if __name__ == '__main__':
    unittest.main()

