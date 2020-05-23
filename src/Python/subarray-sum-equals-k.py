#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/subarray-sum-equals-k/

from typing import List
import unittest


class Solution:
    def subarraySum(self, nums: List[int], k: int) -> int:
        """
        This approach uses cumulative sums. Each element `acc[i]`
        contains the sum of all the elements up until `i`. In order
        to determine the sum between (start, end) we just need to
        calculate sum(0, end) - sum(0, start). Each pair (start, end)
        whose `sum` == k is a candidate contiguous array.

        This solution implemented in Python is not accepted (TLE), but
        the same logic implemented in Rust is.

        Time complexity: O(n^2)
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
        """
        Let `sum[]` be an array of cumulative sums, where `sum[i]`
        is the sum of all the elements of the array up until index
        `i` (just like in the previous solution).
        We can then assume that if `sum[i] == sum[j]`, then the sum
        of all elements between `i` and `j` is 0 (zero). More
        generically: if `sum[j] - sum[i] == k` (for i != j), then the
        sum of all the elements between `i` and `j` is k.
        With this in mind, we can build a hashtable that maps every
        cumulative sum to its number of occurrences. The number of
        subarrays whose sum is K is given by the sum of all
        `occur(sum_i - k)`, where `sum_i` is a distinct cumulative sum
        and `occur(sum_i - k)` is the number of occurrences of `sum_i - k`
        in the hashtable.

        Time complexity: O(n)
        """
        occur = {0: 1}

        count= _sum= 0
        for i in range(len(nums)):
            _sum += nums[i]
            if _sum - k in occur:
                count += occur[_sum - k]

            occur[_sum] = occur.get(_sum, 0) + 1

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
            self.assertEqual(expected, s.subarraySum2(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
