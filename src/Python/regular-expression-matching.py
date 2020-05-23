#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/regular-expression-matching/
import unittest


class Solution(object):
    def isMatch(self, s, p):
        """
        :type s: str
        :type p: str
        :rtype: bool
        """
        if not p:
            return not s
        match = bool(s) and (p[0] == s[0] or p[0] == '.')
        if len(p) > 1 and p[1] == '*':
            return self.isMatch(s, p[2:]) or \
                (match and self.isMatch(s[1:], p))
        else:
            return match and self.isMatch(s[1:], p[1:])


class TestSolution(unittest.TestCase):
    def test_isMatch(self):
        s = Solution()
        self.assertFalse(s.isMatch('aa', 'a'))
        self.assertTrue(s.isMatch('aa', 'a*'))
        self.assertTrue(s.isMatch('ab', '.*'))
        self.assertTrue(s.isMatch('aab', 'c*a*b'))
        self.assertFalse(s.isMatch('mississippi', 'mis*is*p*.'))
        self.assertTrue(s.isMatch('mississippi', '.*'))
        self.assertFalse(s.isMatch('mississippi', '.'))
        self.assertTrue(s.isMatch('aaa', 'a*a'))
        self.assertFalse(s.isMatch('ab', '.*c'))


if __name__ == '__main__':
    unittest.main()

