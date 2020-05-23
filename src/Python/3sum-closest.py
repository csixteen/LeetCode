#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/3sum-closest/
import unittest


class Solution(object):
    def _trim(self, nums):
        ret = []
        count = 0
        current = None
        for i in nums:
            if i == current and count >= 3:
                continue
            elif i == current and count < 3:
                count += 1
            elif i != current:
                current = i
                count = 1
            ret.append(i)
        return ret

    def _threeSumClosest(self, nums, target):
        """
        :type nums: list[int]
        :type target: int
        :rtype: int
        """
        m = None
        for i in range(len(nums)-2):
            left, right = i+1, len(nums)-1
            while left < right:
                s = nums[i] + nums[left] + nums[right]
                if m is None or \
                        abs(s - target) < abs(m - target):
                    m = s
                if s < target:
                    left += 1
                elif s > target:
                    right -= 1
                else:
                    return target

        return m

    def threeSumClosest(self, nums, target):
        """
        :type nums: list[int]
        :type target: int
        :rtype: int
        """
        nums.sort()
        if (nums[0] > 0 and target <= 0) or \
                (nums[-1] < 0 and target >= 0):
            return sum(nums[-3:])
        else:
            return self._threeSumClosest(self._trim(nums), target)


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                ([-1, 2, 1, -4], 1),
                2
            ),
            (
                ([-1, 0, 1, 2, -1, -4], 3),
                3
            ),
            (
                ([-1, 0, 1, 2, -1, -4], 2),
                2
            ),
            (
                ([-1, 4, 0, 1, 2, -1, -4, 3], 10),
                9
            )
        ]

    def test_threeSumClosest(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(s.threeSumClosest(*i), e)


if __name__ == '__main__':
    unittest.main()

