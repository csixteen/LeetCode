#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/merge-two-sorted-lists
import unittest

from lists import (
    ListNode,
    to_linked_list
)


class Solution(object):
    def mergeTwoLists(self, l1, l2):
        """
        :type l1: ListNode
        :type l2: ListNode
        :rtyle: ListNode
        """
        l3, ret = None,  None
        while not (l1 is None and l2 is None):
            choose_l1 = (l2 is None) or \
                (l1 is not None and l2 is not None and l1.val < l2.val)
            if choose_l1:
                tmp = l1.val
                l1 = l1.next
            else:
                tmp = l2.val
                l2 = l2.next

            if ret is None:
                l3 = ListNode(tmp)
                ret = l3
            else:
                l3.next = ListNode(tmp)
                l3 = l3.next
        return ret


class TestSolution(unittest.TestCase):
    def _validate_list(self, l, size):
        c = 0
        previous = None
        while l is not None:
            if previous is None:
                previous = l.val
            else:
                self.assertLessEqual(previous, l.val)
            l = l.next
            c += 1
        self.assertEqual(c, size)

    def test_mergeTwoLists(self):
        s = Solution()
        l1 = to_linked_list([1, 2, 4])
        l2 = to_linked_list([1, 3, 4])
        l3 = s.mergeTwoLists(l1, l2)
        self._validate_list(l3, 6)

        l1 = to_linked_list([])
        l2 = to_linked_list([1, 2, 3])
        l3 = s.mergeTwoLists(l1, l2)
        self._validate_list(l3, 3)

        l1 = to_linked_list([])
        l2 = to_linked_list([])
        l3 = s.mergeTwoLists(l1, l2)
        self._validate_list(l3, 0)

        l1 = to_linked_list([-1, 0, 1])
        l2 = to_linked_list([-1, 0, 1])
        l3 = s.mergeTwoLists(l1, l2)
        self._validate_list(l3, 6)


if __name__ == '__main__':
    unittest.main()
