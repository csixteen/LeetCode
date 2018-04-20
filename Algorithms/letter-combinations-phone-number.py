#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/letter-combinations-of-a-phone-number/
import itertools
import unittest


class Solution(object):
    def letterCombinations(self, digits):
        """
        :type digits: str
        :rtype: list[str]
        """
        if not len(digits):
            return []

        keyboard = {
            '2': ['a', 'b', 'c'],
            '3': ['d', 'e', 'f'],
            '4': ['g', 'h', 'i'],
            '5': ['j', 'k', 'l'],
            '6': ['m', 'n', 'o'],
            '7': ['p', 'q', 'r', 's'],
            '8': ['t', 'u', 'v'],
            '9': ['w', 'x', 'y', 'z']
        }
        tuples = list(itertools.product(*[keyboard[d] for d in digits]))
        return [''.join(x) for x in tuples]


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                '23',
                ['ad', 'ae', 'af', 'bd', 'be', 'bf', 'cd', 'ce', 'cf']
            )
        ]

    def test_letterCombinations(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(s.letterCombinations(i), e)


if __name__ == '__main__':
    unittest.main()

