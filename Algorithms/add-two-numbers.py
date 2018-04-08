#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/add-two-numbers
import unittest


class ListNode(object):
    def __init__(self, x):
        self.val = x
        self.next = None


def print_list(l):
    """
    :type l: ListNode
    """
    print_list_recursive(l, ' -> ')


def print_list_recursive(l, s):
    """
    :type l: ListNode
    :type s: str
    """
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


def ListNode_to_list(l):
    """
    :type l: ListNode
    :rtype: list
    """
    ret = []
    head = l
    while head is not None:
        ret.append(head.val)
        head = head.next
    return ret


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

        return create_list(ret)


class TestSolution(unittest.TestCase):
    def test_ListNode_to_list(self):
        raw = [1, 2, 3, 4, 5]
        l = create_list(raw)
        lntl = ListNode_to_list(l)
        self.assertEqual(raw, lntl)
        self.assertEqual('12345', ''.join(str(x) for x in lntl))

        raw = []
        l = create_list(raw)
        lntl = ListNode_to_list(l)
        self.assertEqual(raw, lntl)

    def test_addTwoNumbers(self):
        s = Solution()
        l1 = create_list([2, 4, 3])
        l2 = create_list([5, 6, 4])
        l3 = s.addTwoNumbers(l1, l2)
        self.assertEqual(
            [7, 0, 8],
            ListNode_to_list(l3)
        )

        l1 = create_list([7, 0, 0, 1])
        l2 = create_list([9])
        l3 = s.addTwoNumbers(l1, l2)
        self.assertEqual(
            [6, 1, 0, 1],
            ListNode_to_list(l3)
        )

        l1 = create_list([9, 9, 9])
        l2 = create_list([9, 9, 9])
        l3 = s.addTwoNumbers(l1, l2)
        self.assertEqual(
            [8, 9, 9, 1],
            ListNode_to_list(l3)
        )


if __name__ == '__main__':
    unittest.main()

