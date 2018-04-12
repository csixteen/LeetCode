#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/add-two-numbers
import unittest

from lists import (
    ListNode,
    to_linked_list,
    to_native
)


class Solution(object):
    def addTwoNumbers(self, l1, l2):
        """
        :type l1: ListNode
        :type l2: ListNode
        :rtype: ListNode
        """
        tmp = 0
        carry = 0
        ret = []
        while not (l1 is None and l2 is None):
            tmp = carry
            carry = 0
            if l1 is not None:
                tmp += l1.val
                l1 = l1.next
            if l2 is not None:
                tmp += l2.val
                l2 = l2.next
            ret.append(tmp % 10)
            carry = tmp // 10

        if carry == 1:
            ret.append(carry)

        return to_linked_list(ret)


class TestSolution(unittest.TestCase):
    def test_addTwoNumbers(self):
        s = Solution()
        l1 = to_linked_list([2, 4, 3])
        l2 = to_linked_list([5, 6, 4])
        l3 = s.addTwoNumbers(l1, l2)
        self.assertEqual(
            [7, 0, 8],
            to_native(l3)
        )

        l1 = to_linked_list([7, 0, 0, 1])
        l2 = to_linked_list([9])
        l3 = s.addTwoNumbers(l1, l2)
        self.assertEqual(
            [6, 1, 0, 1],
            to_native(l3)
        )

        l1 = to_linked_list([9, 9, 9])
        l2 = to_linked_list([9, 9, 9])
        l3 = s.addTwoNumbers(l1, l2)
        self.assertEqual(
            [8, 9, 9, 1],
            to_native(l3)
        )


if __name__ == '__main__':
    unittest.main()

