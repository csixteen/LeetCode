#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/roman-to-integer
import unittest


class Solution(object):
    def romanToInt(self, s):
        """
        :type s: str
        :rtype: int
        """
        values = {'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}
        next_order = {'I': ['V', 'X'], 'X': ['L', 'C'], 'C': ['D', 'M']}

        i, total = 0, 0
        previous_c = None
        while i < len(s):
            if i < len(s)-1 and \
                    (s[i] in next_order) and \
                    (s[i+1] in next_order[s[i]]):
                if previous_c != s[i]:
                    tmp = values[s[i+1]] - values[s[i]]
                    step = 2
            else:
                tmp = values[s[i]]
                step = 1
            total += tmp
            i += step
        return total


class TestSolution(unittest.TestCase):
    def test_romanToInt(self):
        s = Solution()
        self.assertEqual(s.romanToInt('IX'), 9)
        self.assertEqual(s.romanToInt('MCDXCIX'), 1499)
        self.assertEqual(s.romanToInt('III'), 3)
        self.assertEqual(s.romanToInt('XIX'), 19)
        self.assertEqual(s.romanToInt('MIII'), 1003)
        self.assertEqual(s.romanToInt('MDCCCLXXXIV'), 1884)


if __name__ == '__main__':
    unittest.main()
