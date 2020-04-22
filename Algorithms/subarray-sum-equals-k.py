#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/subarray-sum-equals-k/

from typing import List
import unittest


class Solution:
    def subarraySum(self, nums: List[int], k: int) -> int:
        """
        Time complexity: O(n^2)
        This approach uses cumulative sums. Each element `acc[i]`
        contains the sum of all the elements up until `i`. In order
        to determine the sum between (start, end) we just need to
        calculate sum(0, end) - sum(0, start). Each pair (start, end)
        whose `sum` == k is a candidate contiguous array.

        This solution implemented in Python is not accepted (TLE), but
        the same logic implemented in Rust is.
        """
        acc = [0]

        for i in range(1, len(nums) + 1):
            acc.append(acc[i-1] + nums[i-1])

        count = 0
        for i in range(len(nums)):
            for j in range(i+1, len(nums) + 1):
                if acc[j] - acc[i] == k:
                    count += 1

        return count

    def subarraySum2(self, nums: List[int], k: int) -> int:
        count = 0
        return count


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"nums": [1, 1, 1], "k": 2},
                2,
            ),
            (
                {"nums": [49, -2, 10, 0, 8, -6, 6, 102, -94, 0, 1], "k": 8},
                10,
            ),
            (
                {"nums": [0, 1, 2, 3, 4, 5, 6, 0], "k": 6},
                4,
            ),
        ]

    def test_subarraySum(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.subarraySum(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
