#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/rotate-array/
import unittest


class Solution(object):
    def rotate(self, nums, k):
        """
        :type nums: list[int]
        :type k: int
        :rtype: void
        """
        n = len(nums)
        if n > 0:
            nums[:] = nums[-k % n:] + nums[:-k % n]


class TestSolution(unittest.TestCase):
    def test_rotate(self):
        s = Solution()
        i = [1,2,3,4,5,6,7]
        s.rotate(i, 3)
        self.assertEqual(i, [5,6,7,1,2,3,4])

        i = [-1,-100,3,99]
        s.rotate(i, 2)
        self.assertEqual(i, [3,99,-1,-100])

        i = []
        s.rotate(i, 3)
        self.assertEqual(i, [])

        i = [1]
        s.rotate(i, 0)
        self.assertEqual(i, [1])

        i = [1, 2]
        s.rotate(i, 3)
        self.assertEqual(i, [2, 1])


if __name__ == '__main__':
    unittest.main()

