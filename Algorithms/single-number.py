#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/single-number/
from collections import defaultdict
import unittest


class Solution(object):
    def singleNumber(self, nums):
        """
        :type nums: list[int]
        :rtype: int
        """
        d = defaultdict(int)
        for i in nums:
            d[i] += 1

        for k, v in d.items():
            if v == 1:
                return k


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            ([2, 2, 1], 1),
            ([4,1,2,1,2], 4)
        ]

    def test_singleNumber(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(s.singleNumber(i), e)


if __name__ == '__main__':
    unittest.main()

