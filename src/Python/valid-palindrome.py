#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/valid-palindrome/
import re
import unittest


class Solution(object):
    def isPalindrome(self, s):
        """
        :type s: str
        :rtype: bool
        """
        new_str = re.sub(r'[^a-zA-Z0-9]', '', s).lower()
        return new_str == new_str[::-1]


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            ('A man, a plan, a canal: Panama', True),
            ('race a car', False)
        ]

    def test_isPalindrome(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(s.isPalindrome(i), e)


if __name__ == '__main__':
    unittest.main()

