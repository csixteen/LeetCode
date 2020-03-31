#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/

from typing import List
import unittest


class Solution:
    def extreme_insertion_index(self, nums: List[int], target: int, left: bool) -> int:
        lo, hi = 0, len(nums)

        while lo < hi:
            mid = lo + (hi - lo) // 2

            if nums[mid] > target or (left and target == nums[mid]):
                hi = mid
            else:
                lo = mid + 1

        return lo

    def searchRange(self, nums: List[int], target: int) -> List[int]:
        left_idx = self.extreme_insertion_index(nums, target, True)

        if left_idx == len(nums) or nums[left_idx] != target:
            return [-1, -1]

        return [
            left_idx,
            self.extreme_insertion_index(nums, target, False) - 1,
        ]


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (([5, 7, 7, 8, 8, 10], 8), [3, 4]),
            (([5, 7, 7, 8, 8, 10], 6), [-1, -1]),
            (([5, 7, 7, 8, 8, 8, 8, 10], 8), [3, 6]),
        ]

    def test_searchRange(self):
        s = Solution()

        for _input, _expected in self.input_expected:
            res = s.searchRange(*_input)
            self.assertEqual(
                _expected,
                res,
                f"Input: {_input}, Expected: {_expected}, got {res} instead.",
            )
            # self.assertTrue(_expected[0] <= s.searchRange(*_input) <= _expected[1])


if __name__ == "__main__":
    unittest.main(verbosity=2)
