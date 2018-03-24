#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution(object):
    def findMedianSortedArrays(self, nums1, nums2):
        """
        :type nums1: list[int]
        :type nums2: list[int]
        :rtype: float
        """
        # First, we merge the two sorted arrays ( O(N + M) )
        nums3 = []
        N, M = len(nums1), len(nums2)
        i, j = 0, 0
        for _ in range(N + M):
            if i >= N:
                nums3.append(nums2[j])
                j += 1
            elif j >= M:
                nums3.append(nums1[i])
                i += 1
            elif nums1[i] > nums2[j]:
                nums3.append(nums2[j])
                j += 1
            else:
                nums3.append(nums1[i])
                i += 1

        if (N + M) % 2 == 1:
            return nums3[(N + M) // 2]
        else:
            return sum(nums3[(N + M) // 2 - 1:(N + M) // 2 + 1]) / 2


class TestSolution(unittest.TestCase):
    def test_findMedianSortedArrays(self):
        s = Solution()
        self.assertEqual(
            s.findMedianSortedArrays([1, 3], [2]),
            2.0
        )
        self.assertEqual(
            s.findMedianSortedArrays([1, 2], [3, 4]),
            2.5
        )


if __name__ == '__main__':
    unittest.main()

