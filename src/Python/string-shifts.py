#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/529/week-2/3299/

from typing import List
import unittest


class Solution:
    def stringShift(self, s: str, shift: List[List[int]]) -> str:
        shifts = 0

        for direction, amount in shift:
            if direction == 0:
                shifts += amount
            else:
                shifts -= amount

        shifts = shifts % len(s)

        return s[shifts:] + s[:shifts]


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"s": "abc", "shift": [[0,1],[1,2]]},
                "cab",
            ),
            (
                {"s": "abcdefg", "shift": [[1,1],[1,1],[0,2],[1,3]]},
                "efgabcd",
            ),
            (
                {"s": "abcd", "shift": [[0, 1], [0, 1], [0, 2]]},
                "abcd",
            ),
            (
                {"s": "abcd", "shift": [[1, 1], [1, 1]]},
                "cdab",
            ),
        ]

    def test_stringShift(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.stringShift(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
