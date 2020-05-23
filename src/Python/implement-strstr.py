#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/implement-strstr
import unittest


class Solution(object):
    def strStr(self, haystack, needle):
        """
        :type haystack: str
        :type needle: str
        :rtype: int
        """
        if needle == '':
            return 0

        len_haystack = len(haystack)
        len_needle = len(needle)
        if len_haystack == 0 or \
                len_needle == 0 or \
                len_needle > len_haystack:
            return -1

        start, end = -1, -1
        for i in range(len_haystack - len_needle + 1):
            if haystack[i] == needle[0]:
                start = i
                break

        if start == -1:
            return -1

        for i in range(len_haystack - 1, len_haystack - len_needle - 1, -1):
            if haystack[i] == needle[len_needle - 1]:
                end = i
                break

        if end == -1:
            return -1

        if end - start + 1 < len_needle:
            return -1

        i = start
        while i <= end:
            if needle == haystack[i:i + len_needle]:
                return i
            i += 1

        return -1


class TestSolution(unittest.TestCase):
    def test_strStr(self):
        s = Solution()
        self.assertEqual(s.strStr('a', 'a'), 0)
        self.assertEqual(s.strStr('hello', 'll'), 2)
        self.assertEqual(s.strStr('aaaaa', 'bba'), -1)
        self.assertEqual(s.strStr('aaaaaba', 'ab'), 4)
        self.assertEqual(s.strStr('aaaaabababbbbbab', 'ab'), 4)
        self.assertEqual(s.strStr('aaaab', 'b'), 4)


if __name__ == '__main__':
    unittest.main()

