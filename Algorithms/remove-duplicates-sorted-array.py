#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution(object):
    def removeDuplicates(self, nums):
        """
        :type nums: list[int]
        :rtype: int
        """
        previous = None
        i, steps, l = 0, 0, len(nums)
        while i < l:
            if nums[i] != previous:
                previous = nums[i]
                nums[i-steps] = nums[i]
                i += 1
            else:
                while i < l  and nums[i] == previous:
                    steps += 1
                    i += 1
        return l - steps


class TestSolution(unittest.TestCase):
    def test_removeDuplicates(self):
        s = Solution()
        a = [1, 1, 2, 3]
        ret = s.removeDuplicates(a)
        self.assertEqual(3, ret)
        self.assertEqual([1, 2, 3, 3], a)

        a = []
        ret = s.removeDuplicates(a)
        self.assertEqual(0, ret)
        self.assertEqual([], a)

        a = [1, 2, 3, 4, 5]
        ret = s.removeDuplicates(a)
        self.assertEqual(5, ret)
        self.assertEqual([1, 2, 3, 4, 5], a)

        a = [1, 1, 1, 1, 1]
        ret = s.removeDuplicates(a)
        self.assertEqual(1, ret)
        self.assertEqual([1, 1, 1, 1, 1], a)

        a = [1, 2, 3, 4, 4, 4]
        ret = s.removeDuplicates(a)
        self.assertEqual(4, ret)
        self.assertEqual([1, 2, 3, 4, 4, 4], a)


if __name__ == '__main__':
    unittest.main()

