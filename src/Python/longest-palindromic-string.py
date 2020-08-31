#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/longest-palindromic-substring
import unittest


class Solution(object):
    def _longest_pal(self, s, low, high, i, j, longest_length):
        """
        :type s: str
        :type low: int
        :type high: int
        :type i: int
        :type j: int
        :type longest_length: int
        :rtype: str | None
        """
        while i >= low and j <= high:
            if s[i] != s[j]:
                break
            i -= 1
            j += 1

        if len(s[i+1:j]) > longest_length:
            return s[i+1:j]
        return None

    def longestPalindrome(self, s):
        """
        :type s: str
        :rtype: str
        """
        l = len(s)
        if l == 0:
            return ''
        elif l == 1:
            return s

        longest = s[0]
        # Not proud of this code...at all!!!
        for x in range(l):
            i = j = x
            offset = min(i - 0, l - j - 1)
            res = self._longest_pal(
                s, i - offset, j + offset, i, j, len(longest)
            )
            if res is not None:
                longest = res

            if j < l - 1 and s[i] == s[j+1]:
                j += 1
                offset = min(i - 0, l - j - 1)
                res = self._longest_pal(
                    s, i - offset, j + offset, i, j, len(longest)
                )

                if res is not None:
                    longest = res

        return longest


class TestSolution(unittest.TestCase):
    def test_longestPalindrome(self):
        s = Solution()
        self.assertEqual(s.longestPalindrome('babad'), 'bab')
        self.assertEqual(s.longestPalindrome('cbbd'), 'bb')
        self.assertEqual(s.longestPalindrome('bbbbbb'), 'bbbbbb')
        self.assertEqual(s.longestPalindrome('abcdefedbbbb'), 'defed')
        self.assertEqual(s.longestPalindrome(''), '')
        self.assertEqual(s.longestPalindrome('c'), 'c')
        self.assertEqual(s.longestPalindrome('abcdef'), 'a')
        self.assertEqual(s.longestPalindrome('ccc'), 'ccc')


if __name__ == '__main__':
    unittest.main()

