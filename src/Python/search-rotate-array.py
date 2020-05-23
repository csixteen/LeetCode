#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/search-in-rotated-sorted-array/

from typing import List
import unittest


class Solution:
    def search(self, nums: List[int], target: int) -> int:
        lo, hi = 0, len(nums) - 1

        while lo <= hi:
            mid = lo + (hi - lo) // 2

            if nums[mid] == target:
                return mid

            if target > nums[mid]:
                if nums[hi] >= target or nums[hi] < nums[mid] < target:
                    lo = mid + 1
                elif nums[lo] <= target:
                    hi = mid - 1
                else:
                    return -1
            else:
                if nums[lo] <= target or nums[lo] > nums[mid]:
                    hi = mid - 1
                elif nums[hi] >= target:
                    lo = mid + 1
                else:
                    return -1

        return -1


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"nums": [4, 5, 6, 7, 8, 1, 2, 3], "target": 8},
                4,
            ),
            (
                {"nums": [3, 1], "target": 1},
                1,
            ),
            (
                {"nums": [4, 5, 6, 7, 0, 1, 2], "target": 4},
                0,
            ),
            (
                {"nums": [4, 5, 6, 7, 0, 1, 2], "target": 3},
                -1
            ),
            (
                {"nums": [4, 5, 6, 7, 0, 1, 2], "target": 6},
                2,
            ),
            (
                {"nums": [4, 5, 6, 7, 0, 1, 2], "target": 1},
                5,
            ),
            (
                {"nums": [4, 5, 6, 7, 0, 1, 2], "target": 2},
                6,
            ),
            (
                {"nums": [0, 1, 2, 4, 5, 6, 7], "target": 2},
                2,
            ),
            (
                {"nums": [101, 103, 106, 109, 158, 164, 182, 187, 202, 205, 2, 3, 32, 57, 69, 74, 81, 99, 100], "target": 202},
                8,
            ),
            (
                {"nums": [180, 181, 182, 183, 184, 187, 188, 189, 191, 192, 193, 194, 195, 196, 201, 202, 203, 204, 3, 4, 5, 6, 7, 8, 9, 10, 14, 16, 17, 18, 19, 23, 26, 27, 28, 29, 32, 33, 36, 37, 38, 39, 41, 42, 43, 45, 48, 51, 52, 53, 54, 56, 62, 63, 64, 67, 69, 72, 73, 75, 77, 78, 79, 83, 85, 87, 90, 91, 92, 93, 96, 98, 99, 101, 102, 104, 105, 106, 107, 108, 109, 111, 113, 115, 116, 118, 119, 120, 122, 123, 124, 126, 127, 129, 130, 135, 137, 138, 139, 143, 144, 145, 147, 149, 152, 155, 156, 160, 162, 163, 164, 166, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177], "target": 42},
                43,
            ),
        ]

    def test_search(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.search(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
