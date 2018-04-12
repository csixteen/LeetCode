#!/usr/bin/env python3
# coding: utf-8
import unittest


class ListNode(object):
    def __init__(self, v):
        self.val = v
        self.next = None


def to_linked_list(l):
    """
    :type l: list
    :rtype: ListNode | None
    """
    if not l:
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
    while l:
        ret.append(l.val)
        l = l.next
    return ret


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
    if l:
        print(l.val, end='')
        print(s, end='')
        print_list_recursive(l.next, s)
    else:
        print()


class TestLists(unittest.TestCase):
    def test_to_linked_list(self):
        self.assertEqual(
            to_native(to_linked_list([1, 1, 2, 3, 4])),
            [1, 1, 2, 3, 4]
        )
        self.assertEqual(
            to_native(to_linked_list([1, 2, 3, 4, 5])),
            [1, 2, 3, 4, 5]
        )


if __name__ == '__main__':
    unittest.main()

