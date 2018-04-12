#!/usr/bin/env python3
# coding: utf-8
import unittest

from lists import (
    ListNode,
    to_linked_list,
    to_native
)


class Solution(object):
    def mergeTwoLists(self, l1=None, l2=None):
        """
        :type l1: ListNode
        :type l2: ListNode
        :rtyle: ListNode
        """
        l3, ret = None, None
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

    def mergeKLists(self, lists):
        """
        :type lists: list[ListNode]
        :rtype: ListNode
        """
        if len(lists) <= 2:
            return self.mergeTwoLists(*lists)

        mid = len(lists) // 2
        left = self.mergeKLists(lists[:mid])
        right = self.mergeKLists(lists[mid:])

        return self.mergeTwoLists(left, right)


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                [
                    to_linked_list([1, 4, 10]),
                    to_linked_list([1, 2, 3]),
                    to_linked_list([6, 7, 9])
                ],
                [1, 1, 2, 3, 4, 6, 7, 9, 10]
            ),
            (
                [],
                []
            ),
            (
                [
                    to_linked_list([1, 1]),
                    to_linked_list([1, 1]),
                    to_linked_list([1, 1]),
                    to_linked_list([1, 1]),
                    to_linked_list([1, 1])
                ],
                [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
            )
        ]

    def test_mergeKLists(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(to_native(s.mergeKLists(i)), e)


if __name__ == '__main__':
    unittest.main()

