#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/product-of-array-except-self/

from itertools import starmap
from typing import List
import unittest


class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        left = [1] * len(nums)
        right = [1] * len(nums)

        for i in range(1, len(nums)):
            left[i] = left[i-1] * nums[i-1]

        for i in reversed(range(len(nums) - 1)):
            right[i] = right[i+1] * nums[i+1]

        return list(starmap(int.__mul__, zip(left, right)))


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"nums": [1, 2, 3, 4]},
                [24, 12, 8, 6],
            ),
            (
                {"nums": [0, 1, 2, 3, 4]},
                [24, 0, 0, 0, 0],
            ),
            (
                {"nums": [1, 2, 3, 4, 5]},
                [120, 60, 40, 30, 24],
            ),
        ]

    def test_productExceptSelf(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.productExceptSelf(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
