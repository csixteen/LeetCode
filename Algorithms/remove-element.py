#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution(object):
    def _find_next_pivot(self, array, elem, start, end):
        for i in range(end, start-1, -1):
            if array[i] != elem:
                return i
        return -1

    def removeElement(self, nums, val):
        """
        :type nums: list[int]
        :type val: int
        :rtype: int
        """
        count, i, l = 0, 0, len(nums)
        last_index = l -1
        while i < l:
            if nums[i] == val:
                pivot = self._find_next_pivot(nums, val, i, last_index)
                if pivot != -1:
                    nums[i] = nums[pivot]
                    last_index = pivot - 1
                count += 1
            i += 1
        return l - count


class TestSolution(unittest.TestCase):
    def test_removeElement(self):
        s = Solution()
        l = [3, 2, 2, 3]
        ret = s.removeElement(l, 3)
        self.assertEqual(ret, 2)
        self.assertEqual(l[:ret].count(3), 0)

        l = [1, 2, 3, 4, 4, 4, 4]
        ret = s.removeElement(l, 4)
        self.assertEqual(ret, 3)
        self.assertEqual(l[:ret].count(4), 0)

        l = [4, 4, 4, 4, 1, 2, 2, 3, 4]
        ret = s.removeElement(l, 4)
        self.assertEqual(ret, 4)
        self.assertEqual(l[:ret].count(4), 0)

        l = [1, 2, 3, 4]
        ret = s.removeElement(l, 1)
        self.assertEqual(ret, 3)
        self.assertEqual(l[:ret].count(1), 0)


if __name__ == '__main__':
    unittest.main()

