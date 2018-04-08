#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/palindrome-number
import math
import unittest


class Solution(object):
    def isPalindrome(self, x):
        """
        :type x: int
        :rtype: bool
        """
        if x == 0:
            return True
        if x < 0:
            return False

        n_digits = int(math.log10(x)) + 1
        while n_digits // 2 > 0:
            if (x // 10**(n_digits-1)) != (x % 10):
                return False
            x = (x // 10) % (10**(n_digits-2))
            n_digits -= 2
        return True


class TestSolution(unittest.TestCase):
    def test_isPalindrome(self):
        s = Solution()
        self.assertTrue(s.isPalindrome(0))
        self.assertTrue(s.isPalindrome(12321))
        self.assertTrue(s.isPalindrome(1))
        self.assertTrue(s.isPalindrome(99876967899))
        self.assertFalse(s.isPalindrome(-12345))
        self.assertFalse(s.isPalindrome(-1234321))
        self.assertFalse(s.isPalindrome(12345))
        self.assertFalse(s.isPalindrome(12))


if __name__ == '__main__':
    unittest.main()
