#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/jump-game/

from typing import List
import unittest


class Solution:
    def canJump(self, nums: List[int]) -> bool:
        i= span= 0
        last = len(nums) - 1

        while i < last:
            if i > span:
                return False

            span = max(span, i + nums[i])
            i += 1

        return last <= span


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"nums": [0]},
                True,
            ),
            (
                {"nums": [0, 1]},
                False,
            ),
            (
                {"nums": [2, 5, 0, 0]},
                True,
            ),
            (
                {"nums": [2, 0, 0]},
                True,
            ),
            (
                {"nums": [2, 3, 1, 1, 4]},
                True,
            ),
            (
                {"nums": [3, 2, 1, 0, 4]},
                False,
            ),
            (
                {"nums": [1, 4, 0, 0, 0, 3, 1, 0, 4]},
                True,
            ),
            (
                {"nums": [5, 0, 0, 0, 0, 3, 1, 0, 4]},
                True,
            ),
            (
                {"nums": [4, 3, 0, 0, 0, 3, 1, 0, 4]},
                False,
            ),
        ]

    def test_canJump(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.canJump(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
