#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/two-sum-ii-input-array-is-sorted/
import unittest


class Solution(object):
    def twoSum(self, numbers, target):
        """
        :type numbers: list[int]
        :type target: int
        :rtype: list[int]
        """
        i, right = 0, len(numbers)-1
        while i < right:
            s = numbers[i] + numbers[right]
            if s == target:
                return [i+1, right+1]
            if s < target:
                i += 1
            else:
                right -= 1


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                ([-1, 0], -1),
                [1, 2]
            ),
            (
                ([2, 7, 11, 15], 9),
                [1, 2]
            ),
            (
                ([2, 7, 11, 15], 13),
                [1, 3]
            )
        ]

    def test_twoSum(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(s.twoSum(*i), e)


if __name__ == '__main__':
    unittest.main()

