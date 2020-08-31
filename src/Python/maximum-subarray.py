#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/maximum-subarray

from typing import List
import unittest


class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        """
        :type nums: list[int]
        :rtype: int
        """
        _global = max(nums)
        _local = 0

        for i in nums:
            _local = i + max(_local, 0)
            _global = max(_global, _local)

        return _global


class TestSolution(unittest.TestCase):
    def test_maxSubArray(self):
        s = Solution()

        self.assertEqual(s.maxSubArray([-2,1,-3,4,-1,2,1,-5,4]), 6)
        self.assertEqual(s.maxSubArray([-9, -20, -1, -2, -47]), -1)
        self.assertEqual(s.maxSubArray([0]), 0)


if __name__ == '__main__':
    unittest.main()

