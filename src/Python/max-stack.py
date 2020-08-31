#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/max-stack/

import unittest


class MaxStack:
    def __init__(self):
        self.stack = []

    def push(self, x: int) -> None:
        """ Pushes to the top of the stack a value and
        the current maximum value.
        """
        m = max(x, self.stack[-1][1] if len(self.stack) > 0 else float("-inf"))
        self.stack.append((x, m))

    def pop(self) -> int:
        if len(self.stack) == 0:
            raise Exception()

        val, _ = self.stack.pop()
        return val

    def top(self) -> int:
        if len(self.stack) == 0:
            raise Exception()

        return self.stack[-1][0]

    def peekMax(self) -> int:
        if len(self.stack) == 0:
            raise Exception()

        return self.stack[-1][1]

    def popMax(self) -> int:
        """ Uses a temporary stack to store elements that we pop
        from the main stack until we find the maximum element.
        Time complexity: O(n)
        """
        if len(self.stack) == 0:
            raise Exception()

        tmp = []
        while self.stack[-1][0] != self.stack[-1][1]:
            tmp.append(self.stack.pop())

        _, old_max = self.stack.pop()

        for elem, _ in reversed(tmp):
            self.push(elem)

        return old_max


class TestMaxStack(unittest.TestCase):
    def test_MaxStack(self):
        ms = MaxStack()
        ms.push(5)
        ms.push(1)
        ms.push(5)
        self.assertEqual(5, ms.top())
        self.assertEqual(5, ms.popMax())
        self.assertEqual(1, ms.top())
        self.assertEqual(5, ms.peekMax())
        self.assertEqual(1, ms.pop())
        self.assertEqual(5, ms.top())

    def test_failing_test(self):
        ms = MaxStack()

        ms.push(5)
        ms.push(1)
        self.assertEqual(5, ms.popMax())
        self.assertEqual(1, ms.peekMax())


if __name__ == "__main__":
    unittest.main(verbosity=2)
