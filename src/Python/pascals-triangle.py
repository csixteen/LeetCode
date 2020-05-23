#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/pascals-triangle/
import unittest


class Solution(object):
    def generate(self, numRows):
        """
        :type numRows: int
        :rtype: list[list[int]]
        """
        acc = [[1], [1, 1]]
        if numRows == 0:
            return []
        elif numRows == 1:
            return acc[:1]
        elif numRows == 2:
            return acc[:2]
        else:
            for i in range(3, numRows+1):
                row = [1] + \
                    [acc[-1][i] + acc[-1][i+1] for i in range(len(acc[-1])-1)] + \
                    [1]
                acc.append(row)
            return acc


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (1, [[1]]),
            (
                4,
                [[1], [1, 1], [1, 2, 1], [1, 3, 3, 1]]
            ),
            (
                5,
                [[1], [1, 1], [1, 2, 1], [1, 3, 3, 1], [1, 4, 6, 4, 1]]
            )
        ]

    def test_generate(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(s.generate(i), e)


if __name__ == '__main__':
    unittest.main()

