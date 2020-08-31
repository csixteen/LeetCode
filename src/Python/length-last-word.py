#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/length-of-last-word
import re
import unittest


class Solution(object):
    def lengthOfLastWord2(self, s):
        """
        :type s: str
        :rtype: int
        """
        found_word = False
        length = 0
        for i in range(len(s) - 1, -1, -1):
            if s[i] == ' ' and found_word:
                break
            elif s[i] != ' ':
                length += 1
                if not found_word:
                    found_word = True

        return length

    def lengthOfLastWord(self, s):
        """
        :type s: str
        :rtype: int
        """
        _, last_word, _ = re.search(r'^((\w+)?(\s+)?)*$', s).groups()
        return len(last_word) if last_word is not None else 0


class TestSolution(unittest.TestCase):
    def test_lengthOfLastWord(self):
        s = Solution()
        self.assertEqual(s.lengthOfLastWord('Hello World'), 5)
        self.assertEqual(s.lengthOfLastWord('Hello World  '), 5)
        self.assertEqual(s.lengthOfLastWord('a'), 1)
        self.assertEqual(s.lengthOfLastWord('     '), 0)


if __name__ == '__main__':
    unittest.main()

