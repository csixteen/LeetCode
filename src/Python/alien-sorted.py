#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/verifying-an-alien-dictionary/

from enum import Enum
from typing import Dict, List
import unittest


class Order(Enum):
    LT = 1
    EQ = 2
    GT = 2


class Solution:
    def sort_order(self, a: str, b: str, indices: Dict) -> Order:
        len_a = len(a)
        len_b = len(b)

        i = 0
        while i < min(len_a, len_b):
            if indices[a[i]] > indices[b[i]]:
                return Order.GT
            elif indices[a[i]] < indices[b[i]]:
                return Order.LT

            i += 1

        if len_a > len_b:
            return Order.GT
        elif len_a < len_b:
            return Order.LT
        else:
            return Order.EQ

    def isAlienSorted(self, words: List[str], order: str) -> bool:
        if len(words) < 2:
            return True

        indices = dict(zip(order, range(26)))

        for i in range(len(words) - 1):
            order = self.sort_order(words[i], words[i + 1], indices)
            if order == Order.GT:
                return False

        return True


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {
                    "words": ["hello","leetcode"],
                    "order": "hlabcdefgijkmnopqrstuvwxyz",
                },
                True,
            ),
            (
                {
                    "words": ["word","world","row"],
                    "order": "worldabcefghijkmnpqstuvxyz",
                },
                False,
            ),
            (
                {
                    "words": ["apple","app"],
                    "order": "abcdefghijklmnopqrstuvwxyz",
                },
                False,
            ),
        ]

    def test_isAlienSorted(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.isAlienSorted(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
