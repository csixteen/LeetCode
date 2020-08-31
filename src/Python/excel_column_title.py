#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def convertToTitle(self, n: int) -> str:
        ret = []
        
        while n > 0:
            n -= 1
            ret.append(chr((n % 26) + 65))
            n = n // 26

        return "".join(reversed(ret))


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (1, "A"),
            (2, "B"),
            (27, "AA"),
            (28, "AB"),
            (52, "AZ"),
            (53, "BA"),
        ]

    def test_convertToTitle(self):
        for i, e in self.test_cases:
            self.assertEqual(Solution().convertToTitle(i), e)


if __name__ == "__main__":
    unittest.main(verbosity=2)
