#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/pascals-triangle-ii/
import unittest


class Solution(object):
    def getRow(self, rowIndex):
        """
        :type rowIndex: int
        :rtype: list[int]
        """
        if rowIndex == 0:
            return [1]
        elif rowIndex == 1:
            return [1, 1]

        previous = [1, 1]
        for _ in range(2, rowIndex+1):
            previous = [1] + \
                [previous[i]+previous[i+1] for i in range(len(previous)-1)] + \
                [1]
        return previous


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (0, [1]),
            (1, [1, 1]),
            (2, [1, 2, 1]),
            (3, [1, 3, 3, 1]),
            (4, [1, 4, 6, 4, 1])
        ]

    def test_getRow(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(s.getRow(i), e)


if __name__ == '__main__':
    unittest.main()

