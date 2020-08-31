#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/longest-common-prefix
import unittest


class Solution(object):
    def _is_common_char(self, strs, c, i):
        for s in strs:
            if s[i] != c:
                return False
        return True

    def longestCommonPrefix(self, strs):
        """
        :type strs: list[str]
        :rtype: str
        """
        if len(strs) == 0:
            return ''

        longest = []
        shortest_length = min(len(s) for s in strs)
        for i in range(shortest_length):
            c = strs[0][i]
            if not self._is_common_char(strs, c, i):
                break
            else:
                longest.append(c)

        return ''.join(longest)


class TestSolution(unittest.TestCase):
    def test_longestCommonPrefix(self):
        s = Solution()
        self.assertEqual(
            s.longestCommonPrefix([]),
            ''
        )
        self.assertEqual(
            s.longestCommonPrefix(['', '', '']),
            ''
        )
        self.assertEqual(
            s.longestCommonPrefix(['abc', 'def', 'ghi']),
            ''
        )
        self.assertEqual(
            s.longestCommonPrefix(['abc', 'ade', 'agh']),
            'a'
        )
        self.assertEqual(
            s.longestCommonPrefix(['aca', 'cba']),
            ''
        )
        self.assertEqual(
            s.longestCommonPrefix(['aaaaabc', 'aaadef', 'aghi']),
            'a'
        )
        self.assertEqual(
            s.longestCommonPrefix(['aaa', 'aaaaa', 'aaaaaaaaa']),
            'aaa'
        )
        self.assertEqual(
            s.longestCommonPrefix(['aaaa', '']),
            ''
        )


if __name__ == '__main__':
    unittest.main()

