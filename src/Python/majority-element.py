#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/majority-element/
from collections import defaultdict
import unittest


class Solution(object):
    def majorityElement(self, nums):
        """
        :type nums: List[int]
        :rtype: int
        """
        d = defaultdict(int)
        for n in nums:
            d[n] += 1

        c = len(nums) // 2
        for k, v in d.items():
            if v > c:
                return k


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            ([3,2,3], 3),
            ([2,2,1,1,1,2,2], 2)
        ]

    def test_majorityElement(self):
        s = Solution()
        for i,e in self.input_expected:
            self.assertEqual(s.majorityElement(i), e)


if __name__ == '__main__':
    unittest.main()

