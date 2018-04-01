#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution(object):
    def maxSubArray2(self, nums):
        """
        :type nums: list[int]
        :rtype: int
        """
        _global = m = max(nums)
        _local = 0
        for i in nums:
            _local = max(_local + i, min(m, 0))
            _global = max(_global, _local)
        return _global

    def maxSubArray(self, nums):
        """
        :type nums: list[int]
        :rtype: int
        """
        *_, s = self._maxSubArrayRec(nums, 0, len(nums)-1)
        return s

    def _maxSubArrayRec(self, nums, low, high):
        """
        :type nums: list[int]
        :type low: int
        :type high: int
        :rtype: tuple
        """
        if low == high:
            return low, high, nums[low]

        middle = (low + high) // 2
        left_low, left_high, left_sum = \
            self._maxSubArrayRec(nums, low, middle)
        right_low, right_high, right_sum = \
            self._maxSubArrayRec(nums, middle+1, high)
        cross_low, cross_high, cross_sum = \
            self._maxCrossingSubArray(nums, low, middle, high)

        if left_sum >= right_sum and left_sum >= cross_sum:
            return left_low, left_high, left_sum
        elif right_sum >= left_sum and right_sum >= cross_sum:
            return right_low, right_high, right_sum
        else:
            return cross_low, cross_high, cross_sum

    def _maxCrossingSubArray(self, nums, low, middle, high):
        """
        :type nums: list[int]
        :type low: int
        :type middle: int
        :type high: int
        :rtype: tuple
        """
        max_left = max_right = middle
        left_sum = -float('inf')
        s = 0
        for i in range(middle, low-1, -1):
            s += nums[i]
            if s > left_sum:
                left_sum = s
                max_left = i

        right_sum = -float('inf')
        s = 0
        for i in range(middle+1, high+1):
            s += nums[i]
            if s > right_sum:
                right_sum = s
                max_right = i

        return max_left, max_right, left_sum + right_sum


class TestSolution(unittest.TestCase):
    def test_maxSubArray(self):
        s = Solution()
        self.assertEqual(s.maxSubArray([-2,1,-3,4,-1,2,1,-5,4]), 6)
        self.assertEqual(s.maxSubArray2([-2,1,-3,4,-1,2,1,-5,4]), 6)
        self.assertEqual(s.maxSubArray([-9, -20, -1, -2, -47]), -1)
        self.assertEqual(s.maxSubArray2([-9, -20, -1, -2, -47]), -1)
        self.assertEqual(s.maxSubArray([0]), 0)
        self.assertEqual(s.maxSubArray2([0]), 0)


if __name__ == '__main__':
    unittest.main()

