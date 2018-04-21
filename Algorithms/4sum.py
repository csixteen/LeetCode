#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/4sum/
import unittest


class Solution(object):
    def threeSum(self, nums, pivot, target, visited, ret):
        """
        nums is already sorted and shouldn't be empty.

        :type nums: list[int]
        :type pivot: int
        :type target: int
        :rtype: list[list[int]]
        """
        for i in range(len(nums)-2):
            left, right = i+1, len(nums)-1
            while left < right:
                s = nums[i] + nums[left] + nums[right] + pivot
                if s < target:
                    left += 1
                elif s > target:
                    right -= 1
                else:
                    t = pivot, nums[i], nums[left], nums[right]
                    if t not in visited:
                        visited.add(t)
                        ret.append([pivot, nums[i], nums[left], nums[right]])
                    left += 1
                    right -= 1

    def fourSum(self, nums, target):
        """
        :type nums: list[int]
        :type target: int
        :rtype: list[list[int]]
        """
        if not nums:
            return []

        nums.sort()
        ret = []
        visited = set()
        for i in range(len(nums)-3):
            res = self.threeSum(nums[i+1:], nums[i], target, visited, ret)

        return ret


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                ([1, 0, -1, 0, -2, 2], 0),
                [[-2, -1, 1, 2], [-2,  0, 0, 2], [-1, 0, 0, 1]]
            ),
            (
                ([1, -2, -5, -4, -3, 3, 3, 5], -11),
                [[-5, -4, -3, 1]]
            )
        ]

    def test_fourSum(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(s.fourSum(*i), e)


if __name__ == '__main__':
    unittest.main()

