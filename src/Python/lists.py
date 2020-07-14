#!/usr/bin/env python3
# coding: utf-8
import unittest


class ListNode(object):
    def __init__(self, v, next_=None):
        self.val = v
        self.next = next_

    def __str__(self):
        return f'{self.val} ({self.next})'

    def __eq__(self, other):
        if not other:
            return False

        return (self.val == other.val) and (self.next == other.next)


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
    _print_list_recursive(l, ' -> ')


def _print_list_recursive(l, s):
    """
    :type l: ListNode
    :type s: str
    """
    if l:
        print(l.val, end='')
        print(s, end='')
        _print_list_recursive(l.next, s)
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

