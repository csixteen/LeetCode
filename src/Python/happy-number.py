#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/happy-number/

import unittest


class Solution:
    def isHappy(self, n: int) -> bool:
        visited = set()

        while True:
            sum_digits = 0
            
            while n > 0:
                sum_digits += (n % 10)**2
                n //= 10

            if sum_digits in visited:
                return False

            if sum_digits == 1:
                return True

            visited.add(sum_digits)
            n = sum_digits


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            #(19, True),
            #(18, False),
            #(82, True),
            (7, True),
        ]

    def test_isHappy(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.isHappy(_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
