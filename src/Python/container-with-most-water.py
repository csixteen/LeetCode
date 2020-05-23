#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/container-with-most-water/
import unittest


class Solution(object):
    def maxArea(self, height):
        """
        :type height: list[int]
        :rtype: int
        """
        i, j = 0, len(height)-1
        max_area = 0
        while i != j:
            area = (j - i) * min(height[i], height[j])
            if area > max_area:
                max_area = area
            if height[i] < height[j]:
                i += 1
            else:
                j -= 1
        return max_area


class TestSolution(unittest.TestCase):
    def test_maxArea(self):
        s = Solution()
        self.assertEqual(s.maxArea([1, 1]), 1)
        self.assertEqual(s.maxArea([7, 9, 12, 5]), 15)
        self.assertEqual(s.maxArea([1, 8, 6, 2, 5, 4, 8, 3, 7]), 49)
        self.assertEqual(s.maxArea([1, 3, 2, 5, 25, 24, 5]), 24)


if __name__ == '__main__':
    unittest.main()

