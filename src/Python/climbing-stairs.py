#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/climbing-stairs/
import unittest


class Solution(object):
    def climbStairs(self, n):
        """
        :type n: int
        :rtype: int
        """
        a, b, c = 0, 0, 1
        for i in range(1, n+1):
            a = b + c
            b = c
            c = a
        return c


class TestSolution(unittest.TestCase):
    def test_climbStairs(self):
        s = Solution()
        self.assertEqual(s.climbStairs(2), 2)
        self.assertEqual(s.climbStairs(3), 3)
        self.assertEqual(s.climbStairs(4), 5)
        self.assertEqual(s.climbStairs(5), 8)
        self.assertEqual(s.climbStairs(6), 13)


if __name__ == '__main__':
    unittest.main()

