#!/usr/bin/env python3
# coding: utf-8
import unittest


class ListNode(object):
    def __init__(self, x):
        self.val = x
        self.next = None


def print_list(l):
    print_list_recursive(l, ' -> ')


def print_list_recursive(l, s):
    if l is not None:
        print(l.val, end='')
        print(s, end='')
        print_list_recursive(l.next, s)
    else:
        print()


def create_list(l):
    """
    :type l: list
    :rtype: ListNode
    """
    if len(l) == 0:
        return None

    head = ListNode(l[0])
    ret = head
    for e in l[1:]:
        head.next = ListNode(e)
        head = head.next
    return ret


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

    def test_create_list(self):
        raw = [1, 2, 3, 4, 5]
        l = create_list(raw)
        for e in raw:
            self.assertEqual(e, l.val)
            l = l.next

        raw = []
        l = create_list(raw)
        self.assertIsNone(l)

    def test_mergeTwoLists(self):
        s = Solution()
        l1 = create_list([1, 2, 4])
        l2 = create_list([1, 3, 4])
        l3 = s.mergeTwoLists(l1, l2)
        self._validate_list(l3, 6)

        l1 = create_list([])
        l2 = create_list([1, 2, 3])
        l3 = s.mergeTwoLists(l1, l2)
        self._validate_list(l3, 3)

        l1 = create_list([])
        l2 = create_list([])
        l3 = s.mergeTwoLists(l1, l2)
        self._validate_list(l3, 0)

        l1 = create_list([-1, 0, 1])
        l2 = create_list([-1, 0, 1])
        l3 = s.mergeTwoLists(l1, l2)
        self._validate_list(l3, 6)


if __name__ == '__main__':
    unittest.main()
