#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/reorder-data-in-log-files/

from operator import itemgetter
from typing import List
import re
import unittest


class Solution:
    def reorderLogFiles(self, logs: List[str]) -> List[str]:
        digits = []
        letters = []

        for line in logs:
            res = re.match(r"(\w+)((\s+[a-z]+)+)", line)

            if res is None:
                digits.append(line)
            else:
                letters.append((res.group(1), res.group(2).strip()))

        letters = [" ".join(x) for x in sorted(letters, key=itemgetter(1, 0))]

        return letters + digits


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {
                    "logs": [
                        "dig1 8 1 5 1",
                        "let1 art can",
                        "dig2 3 6",
                        "let2 own kit dig",
                        "let3 art zero"
                    ],
                },
                [
                    "let1 art can",
                    "let3 art zero",
                    "let2 own kit dig",
                    "dig1 8 1 5 1",
                    "dig2 3 6"
                ]
            ),
        ]

    def test_reorderLogFiles(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(
                expected,
                s.reorderLogFiles(**_input),
            )


if __name__ == "__main__":
    unittest.main(verbosity=2)
