#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/remove-duplicates-from-sorted-list
import unittest


class ListNode(object):
    def __init__(self, v):
        self.val = v
        self.next = None

def create_linked_list(l):
    """
    :type l: list
    :rtype: ListNode | None
    """
    if len(l) == 0:
        return None

    head = tmp = ListNode(l[0])
    for v in l[1:]:
        tmp.next = ListNode(v)
        tmp = tmp.next
    return head


def to_native(l):
    """
    :type l: ListNode
    :rtype: list
    """
    ret = []
    while l is not None:
        ret.append(l.val)
        l = l.next
    return ret


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
                create_linked_list([1, 1, 2, 3, 3]),
                [1, 2, 3]
            ),
            (
                create_linked_list([1, 1, 2]),
                [1, 2]
            ),
            (
                create_linked_list([1]),
                [1]
            )
        ]

    def test_create_linked_list(self):
        self.assertEqual(
            to_native(create_linked_list([1, 1, 2, 3, 4])),
            [1, 1, 2, 3, 4]
        )
        self.assertEqual(
            to_native(create_linked_list([1, 2, 3, 4, 5])),
            [1, 2, 3, 4, 5]
        )

    def test_deleteDuplicates(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(to_native(s.deleteDuplicates(i)), e)


if __name__ == '__main__':
    unittest.main()

