#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/3sum/

from typing import List
import unittest


class Solution:
    def threeSum(self, A: List[int]) -> List[List[int]]:
        A.sort()
        if len(A) == 0 or A[0] > 0 or A[-1] < 0:
            return []

        l = len(A)
        res = []
        i = 0
        while i < l - 2:
            left, right = i + 1, l - 1
            while left < right:
                s = A[i] + A[left] + A[right]
                if s > 0:
                    right -= 1
                elif s < 0:
                    left += 1
                else:
                    res.append([A[i], A[left], A[right]])
                    while left < right and A[left] == A[left+1]:
                        left += 1
                    left += 1
                    while right > left and A[right] == A[right-1]:
                        right -= 1
                    right -= 1

            while i < l - 2 and A[i] == A[i+1]:
                i += 1

            i += 1

        return res


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            ([0, 0, 0], [[0, 0, 0]]),
            ([-2, 0, 0, 2, 2], [[-2, 0, 2]]),
            ([-1, 0, 1, 2, -1, -4], [[-1, 0, 1], [-1, -1, 2]]),
            ([-1, 0, -1, -1, 1, 2, -1, -4, -1], [[-1, 0, 1], [-1, -1, 2]]),
            ([-1, -4, 0], []),
            ([200, 1, 0, 17], [])
        ]

    def test_threeSum(self):
        s = Solution()

        for i, e in self.test_cases:
            self.assertEqual(sorted(s.threeSum(i)), sorted(e))


if __name__ == '__main__':
    unittest.main(verbosity=2)

