#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/search-insert-position
import unittest


class Solution(object):
    def _binarySearchInsert(self, nums, target, low, high):
        if high < low:
            return low
        mid = (low + high) // 2
        if nums[mid] > target:
            return self._binarySearchInsert(nums, target, low, mid-1)
        elif nums[mid] < target:
            return self._binarySearchInsert(nums, target, mid+1, high)
        else:
            return mid

    def searchInsert(self, nums, target):
        """
        :type nums: list[int]
        :type target: int
        :rtype: int
        """
        return self._binarySearchInsert(nums, target, 0, len(nums)-1)


class TestSolution(unittest.TestCase):
    def test_searchInsert(self):
        s = Solution()
        self.assertEqual(2, s.searchInsert([1, 3, 5, 6], 5))
        self.assertEqual(1, s.searchInsert([1, 3, 5, 6], 2))
        self.assertEqual(4, s.searchInsert([1, 3, 5, 6], 7))
        self.assertEqual(0, s.searchInsert([1, 3, 5, 6], 0))


if __name__ == '__main__':
    unittest.main()

