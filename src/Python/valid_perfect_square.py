#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    @staticmethod
    def isPerfectSquare(n):
        if n < 2:
            return True

        lo, hi = 2, n //2
        while lo <= hi:
            mid = lo + (hi - lo) // 2
            if mid*mid < n:
                lo = mid + 1
            elif mid*mid > n:
                hi = mid - 1
            else:
                return True

        return False


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (10, False),
            (20, False),
            (36, True),
            (121, True),
            (10000, True),
            (222, False),
            (81, True)
        ]

    def test_isPerfectSquare(self):
        for i, e in self.test_cases:
            self.assertEqual(Solution.isPerfectSquare(i), e)


if __name__ == '__main__':
    unittest.main()

