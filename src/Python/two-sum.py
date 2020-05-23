#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/two-sum/

from typing import List
import unittest


class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        indices = {}

        for i, n in enumerate(nums):
            x = target - n
            if x in indices:
                return [indices[x], i]

            indices[n] = i


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (([2, 7, 11, 15], 9), [0, 1]),
            (([101, 23, 45, 3, 66, 14], 68), [1, 2]),
            (([999, 1, 132, 57, 192, 8, 54], 55), [1, 6]),
        ]

    def test_twoSum(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(s.twoSum(*_input), expected)


if __name__ == "__main__":
    unittest.main(verbosity=2)
