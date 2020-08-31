#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/remove-nth-node-from-end-of-list
import unittest

from lists import (
    ListNode,
    to_linked_list,
    to_native
)


class Solution(object):
    def removeNthFromEnd(self, head, n):
        """
        :type head: ListNode
        :type n: int
        :rtype: ListNode
        """
        pointer1 = head
        pointer2 = None
        while pointer1 is not None and n + 1 > 0:
            pointer1 = pointer1.next
            n -= 1

        pointer2 = head
        if pointer1 is None and n + 1 > 0:
            if pointer2.next is None:
                return None
            pointer2.val = pointer2.next.val
            pointer2.next = pointer2.next.next
        else:
            while pointer1 is not None:
                pointer1 = pointer1.next
                pointer2 = pointer2.next
            pointer2.next = pointer2.next.next

        return head


class TestSolution(unittest.TestCase):
    def test_removeNthFromEnd(self):
        input_expected = [
            (
                (to_linked_list([1, 2, 3, 4, 5]), 2),
                [1, 2, 3, 5]
            ),
            (
                (to_linked_list([1, 2, 3, 4, 5]), 1),
                [1, 2, 3, 4]
            ),
            (
                (to_linked_list([1]), 1),
                []
            ),
            (
                (to_linked_list([1, 2]), 1),
                [1]
            ),
            (
                (to_linked_list([1, 2]), 2),
                [2]
            )
        ]
        s = Solution()
        for i, e in input_expected:
            self.assertEqual(to_native(s.removeNthFromEnd(*i)), e)


if __name__ == '__main__':
    unittest.main()

