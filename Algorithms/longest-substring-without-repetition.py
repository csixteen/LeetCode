#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution(object):
    def lengthOfLongestSubstring(self, s):
        """
        :type s: str
        :rtype: int
        """
        i, l = 0, len(s)
        longest = 0
        last_repetition = 0
        while i < l:
            seen = {}
            while i < l and s[i] not in seen:
                seen[s[i]] = i
                i += 1

            if i < l:
                i = seen.get(s[i], i) + 1

            if len(seen) > longest:
                longest = len(seen)

        return longest


class TestSolution(unittest.TestCase):
    def test_lengthOfLongestSubstring(self):
        s = Solution()
        self.assertEqual(s.lengthOfLongestSubstring('abcabcbb'), 3)
        self.assertEqual(s.lengthOfLongestSubstring('bbbbb'), 1)
        self.assertEqual(s.lengthOfLongestSubstring('pwwkew'), 3)
        self.assertEqual(s.lengthOfLongestSubstring('pwwkewilk'), 5)
        self.assertEqual(s.lengthOfLongestSubstring(''), 0)


if __name__ == '__main__':
    unittest.main()

