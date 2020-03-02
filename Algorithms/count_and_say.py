#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/count-and-say/

from itertools import groupby
import unittest


class Solution:
    def countAndSay(self, n: int) -> str:
        ret = "1"

        for i in range(1, n):
            ret = "".join([
                "".join([str(len(list(x[1]))), x[0]])
                for x in groupby(ret)
            ])

        return ret


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (1, "1"),
            (2, "11"),
            (3, "21"),
            (4, "1211"),
            (5, "111221"),
        ]

    def test_countAndSay(self):
        s = Solution()

        for i, e in self.test_cases:
            self.assertEqual(e, s.countAndSay(i))


if __name__ == "__main__":
    unittest.main(verbosity=2)
