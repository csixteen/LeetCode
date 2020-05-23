#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/valid-parenthesis-string/

import unittest


class Solution:
    def checkValidString(self, s: str) -> bool:
        # [lo, hi]
        lo, hi = 0, 0

        for char in s:
            lo += 1 if char == "(" else -1
            hi += 1 if char != ")" else -1

            if hi < 0:
                break

            lo = max(lo, 0)

        return lo == 0


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"s": "()"},
                True,
            ),
            (
                {"s": ")("},
                False,
            ),
            (
                {"s": "(*)"},
                True,
            ),
            (
                {"s": "(*))"},
                True,
            ),
            (
                {"s": "(*())()))"},
                False,
            ),
            (
                {"s": "(**)))"},
                True,
            ),
            (
                {"s": "(((******))"},
                True,
            ),
            (
                {"s": "((()))()(())(*()()())**(())()()()()((*()*))((*()*)"},
                True,
            ),
        ]

    def test_checkValidString(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.checkValidString(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
