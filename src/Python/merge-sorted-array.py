#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution(object):
    def merge(self, nums1, m, nums2, n):
        """
        :type nums1: list[int]
        :type m: int
        :type nums2: list[int]
        :type n: int
        """
        end = m + n - 1
        while m > 0 and n > 0:
            if nums1[m-1] is None or nums2[n-1] > nums1[m-1]:
                nums1[end] = nums2[n-1]
                n -= 1
            else:
                nums1[end] = nums1[m-1]
                nums1[m-1] = None
                m -= 1
            end -= 1
        if n > 0:
            nums1[m:m+n] = nums2[:n]


class TestSolution(unittest.TestCase):
    def test_merge(self):
        s = Solution()
        nums1 = [1, 1, 3, 6, 7, None, None, None, None]
        nums2 = [4, 5, 9, 9]
        s.merge(nums1, 5, nums2, 4)
        self.assertEqual(nums1, [1, 1, 3, 4, 5, 6, 7, 9, 9])

        nums1 = [None, None, None]
        nums2 = [1, 2, 3]
        s.merge(nums1, 0, nums2, 3)
        self.assertEqual(nums1, [1, 2, 3])

        nums1 = [1, 3, 5, 7, 9, None, None, None, None, None]
        nums2 = [2, 4, 6, 8, 10]
        s.merge(nums1, 5, nums2, 5)
        self.assertEqual(nums1, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])

        nums1 = [1, 3, 3, 3, 4, None]
        nums2 = [1]
        s.merge(nums1, 5, nums2, 1)
        self.assertEqual(nums1, [1, 1, 3, 3, 3, 4])


if __name__ == '__main__':
    unittest.main()

