#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/move-zeroes/

from typing import List
import unittest


class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        last_non_zero = 0

        for i in range(len(nums)):
            if nums[i] != 0:
                nums[last_non_zero] = nums[i]
                last_non_zero += 1

        for i in range(last_non_zero, len(nums)):
            nums[i] = 0


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                [0,1,0,3,12],
                [1,3,12,0,0],
            ),
            (
                [1, 2, 3, 4, 0],
                [1, 2, 3, 4, 0],
            ),
            (
                [0, 0, 0],
                [0, 0, 0],
            ),
            (
                [0, 0, 0, 0, 1],
                [1, 0, 0, 0, 0],
            ),
            ([], []),
        ]

    def test_moveZeroes(self):
        s = Solution()

        for _input, expected in self.input_expected:
            s.moveZeroes(_input)
            self.assertEqual(expected, _input)


if __name__ == "__main__":
    unittest.main(verbosity=2)
