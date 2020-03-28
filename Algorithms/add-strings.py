#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/add-strings/

from typing import Tuple
import unittest


class Solution:
    def add_digits(self, a: str, b: str, carry: int) -> Tuple[str, int]:
        s = ord(a) + ord(b) - 2 * ord("0") + carry
        res = s % 10
        carry = s // 10

        return chr(res + ord("0")), carry

    def addStrings(self, num1: str, num2: str) -> str:
        width = max(len(num1), len(num2))
        res = []

        carry = 0
        for a, b in zip(reversed(num1.zfill(width)), reversed(num2.zfill(width))):
            r, carry = self.add_digits(a, b, carry)
            res.append(r)

        if carry > 0:
            res.append(chr(carry + ord("0")))

        return "".join(reversed(res))


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"num1": "5", "num2": "1"},
                "6",
            ),
            (
                {"num1": "5", "num2": "6"},
                "11",
            ),
            (
                {"num1": "1", "num2": "0"},
                "1"
            ),
            (
                {"num1": "99", "num2": "99"},
                "198",
            ),
        ]

    def test_addStrings(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.addStrings(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
