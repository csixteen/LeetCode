#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/valid-parentheses
import unittest


class Solution(object):
    def isValid(self, s):
        """
        :type s: str
        :rtype: bool
        """
        pairs = {'(': ')', '{': '}', '[': ']'}
        opening = []
        count = 0
        for c in s:
            if c in pairs:
                opening.append(c)
                count += 1
            elif len(opening) == 0:
                return False
            else:
                o = opening.pop()
                if c != pairs[o]:
                    return False
                count -= 1
        return count == 0


class TestSolution(unittest.TestCase):
    def test_isValid(self):
        s = Solution()
        self.assertFalse(s.isValid('}][}}(}][))]'))
        self.assertFalse(s.isValid('['))
        self.assertTrue(s.isValid('{}[]()'))
        self.assertFalse(s.isValid('{[}]}'))
        self.assertFalse(s.isValid('(('))
        self.assertTrue(s.isValid('()'))
        self.assertTrue(s.isValid('()[]{}'))
        self.assertFalse(s.isValid('(]'))
        self.assertFalse(s.isValid('([)]'))
        self.assertTrue(s.isValid('(()((())))'))
        self.assertTrue(s.isValid('({})'))


if __name__ == '__main__':
    unittest.main()

