#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/valid-palindrome-ii/

import unittest


class Solution:
    def validPalindrome(self, s: str) -> bool:
        deleted = False
        i, j = 0, len(s) - 1

        while i < j:
            if s[i] != s[j]:
                return s[i+1:j+1] == s[i+1:j+1][::-1] or s[i:j] == s[i:j][::-1]

            i += 1
            j -= 1

        return True


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"s": "aba"},
                True,
            ),
            (
                {"s": "abca"},
                True,
            ),
            (
                {"s": "abcca"},
                True,
            ),
            (
                {"s": "abcdef"},
                False,
            ),
            (
                {"s": "aguokepatgbnvfqmgmlcupuufxoohdfpgjdmysgvhmvffcnqxjjxqncffvmhvgsymdjgpfdhooxfuupuculmgmqfvnbgtapekouga"},
                True,
            ),
        ]

    def test_validPalindrome(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.validPalindrome(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
