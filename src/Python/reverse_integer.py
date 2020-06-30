#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/reverse-integer

import unittest


class Solution:
    def reverse(self, x: int) -> int:
        lo, hi = -2147483648, 2147483647
        stack = []
        neg = -1 if x < 0 else 1
        x = abs(x)

        while x > 0:
            stack.append(x % 10)
            x //= 10

        res, i = 0, 0
        while stack:
            res += stack.pop() * 10**i
            if not (lo <= res * neg <= hi):
                return 0

            i += 1

        return res * neg


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.s = Solution()

    def test_example1(self):
        self.assertEqual(321, self.s.reverse(123))

    def test_example2(self):
        self.assertEqual(-321, self.s.reverse(-123))

    def test_example3(self):
        self.assertEqual(0, self.s.reverse(1111111119))


if __name__ == "__main__":
    unittest.main(verbosity=2)
