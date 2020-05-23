#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/min-stack/
from collections import deque
import unittest


class MinStackElement(object):
    def __init__(self, x):
        self.val = x
        self.min = None


class MinStack(object):
    def __init__(self):
        self.__data = deque()

    def push(self, x):
        """
        :type x: int
        :rtype: void
        """
        elem = MinStackElement(x)
        if len(self.__data) == 0:
            elem.min = x
        else:
            elem.min = min(x, self.__data[-1].min)
        self.__data.append(elem)

    def pop(self):
        """
        :rtype: void
        """
        if len(self.__data) > 0:
            self.__data.pop()

    def top(self):
        """
        :rtype: int
        """
        if len(self.__data) > 0:
            return self.__data[-1].val

    def getMin(self):
        """
        :rtype: int
        """
        if len(self.__data) > 0:
            return self.__data[-1].min


class TestMinStack(unittest.TestCase):
    def test_MinStack(self):
        ms = MinStack()
        ms.push(-2)
        ms.push(0)
        ms.push(-3)
        self.assertEqual(ms.getMin(), -3)
        ms.pop()
        self.assertEqual(ms.top(), 0)
        self.assertEqual(ms.getMin(), -2)
        ms.push(-7)
        ms.push(-7)
        self.assertEqual(ms.getMin(), -7)
        self.assertEqual(ms.top(), -7)
        ms.pop()
        self.assertEqual(ms.getMin(), -7)


if __name__ == '__main__':
    unittest.main()

