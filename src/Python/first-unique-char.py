#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/first-unique-character-in-a-string/

from collections import Counter
import unittest


class Solution:
    def firstUniqChar(self, s: str) -> int:
        counts = Counter(s)
        for i, c in enumerate(s):
            if counts[c] == 1:
                return i

        return -1


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"s": "leetcode"},
                0,
            ),
            (
                {"s": "loveleetcode"},
                2,
            ),
        ]

    def test_firstUniqChar(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.firstUniqChar(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
