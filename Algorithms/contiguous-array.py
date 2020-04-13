#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/contiguous-array/

from typing import List
import unittest


class Solution:
    def findMaxLength(self, nums: List[int]) -> int:
        _max = 0
        count = 0
        count_idx = {}

        for i, n in enumerate(nums):
            count += 1 if n == 1 else -1
            if count == 0:
                _max = max(_max, i + 1)
            elif count in count_idx:
                _max = max(_max, i - count_idx[count])
            else:
                count_idx[count] = i

        return _max


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"nums": [0, 1]},
                2,
            ),
            (
                {"nums": [0, 1, 0]},
                2,
            ),
            (
                {"nums": [1, 0, 1, 0, 1, 0, 1, 0]},
                8,
            ),
            (
                {"nums": [0, 0, 0, 0, 0, 1, 1]},
                4,
            ),
            (
                {"nums": [0, 1, 1, 1, 0, 1, 0, 0, 0]},
                8,
            ),
            (
                {"nums": [0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1]},
                6,
            ),
        ]

    def test_findMaxLength(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.findMaxLength(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
