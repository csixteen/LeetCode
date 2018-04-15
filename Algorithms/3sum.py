#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/3sum/
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

    def _threeSum(self, nums):
        ret = []
        visited = set()
        for i in range(len(nums)-2):
            left, right = i+1, len(nums)-1
            while left < right:
                s = nums[i] + nums[left] + nums[right]
                if s < 0:
                    left += 1
                elif s > 0:
                    right -= 1
                else:
                    t = nums[i], nums[left], nums[right]
                    if t not in visited:
                        visited.add(t)
                        ret.append([nums[i], nums[left], nums[right]])
                    left += 1
                    right -= 1
        return ret

    def threeSum(self, nums):
        """
        :type nums: list[int]
        :rtype: list[list[int]]
        """
        if not nums:
            return []
        nums.sort()
        if nums[0] > 0 or nums[len(nums)-1] < 0:
            return []

        return self._threeSum(self._trim(nums))


class TestSolution(unittest.TestCase):
    def validateTrue(self, l1, l2):
        self.assertEqual(len(l1), len(l2))
        for l in l1:
            self.assertTrue(l in l2)

    def test_threeSum(self):
        s = Solution()
        self.validateTrue(
            s.threeSum([-1, 0, 1, 2, -1, -4]),
            [[-1, 0, 1], [-1, -1, 2]]
        )
        self.validateTrue(
            s.threeSum([0, 1, 2, 3, 4, 5]),
            []
        )
        self.validateTrue(
            s.threeSum([-4, -2, -1]),
            []
        )
        self.validateTrue(
            s.threeSum([-1, 0, 1, 2, -1, -4, 3]),
            [[-1, 0, 1], [-1, -1, 2], [-4, 1, 3]]
        )
        self.validateTrue(
            s.threeSum([-1, 4, 0, 1, 2, -1, -4, 3]),
            [[-1, 0, 1], [-1, -1, 2], [-4, 0, 4], [-4, 1, 3]]
        )
        self.validateTrue(
            s.threeSum([0, 0, 0]),
            [[0, 0, 0]]
        )
        self.validateTrue(
            s.threeSum([-7,-11,12,-15,14,4,4,11,-11,2,-8,5,8,14,0,3,2,3,-3,-15,-2,3,6,1,2,8,-5,-7,3,1,8,11,-3,6,3,-4,-13,-15,14,-8,2,-8,4,-13,13,11,5,0,0,9,-8,5,-2,14,-9,-15,-1,-6,-15,9,10,9,-2,-8,-8,-14,-5,-14,-14,-6,-15,-5,-7,5,-11,14,-7,2,-9,0,-4,-1,-9,9,-10,-11,1,-4,-2,2,-9,-15,-12,-4,-8,-5,-11,-6,-4,-9,-4,-3,-7,4,9,-2,-5,-13,7,2,-5,-12,-14,1,13,-9,-3,-9,2,3,8,0,3]),
            [[-15,1,14],[-15,2,13],[-15,3,12],[-15,4,11],[-15,5,10],[-15,6,9],[-15,7,8],[-14,0,14],[-14,1,13],[-14,2,12],[-14,3,11],[-14,4,10],[-14,5,9],[-14,6,8],[-13,-1,14],[-13,0,13],[-13,1,12],[-13,2,11],[-13,3,10],[-13,4,9],[-13,5,8],[-13,6,7],[-12,-2,14],[-12,-1,13],[-12,0,12],[-12,1,11],[-12,2,10],[-12,3,9],[-12,4,8],[-12,5,7],[-12,6,6],[-11,-3,14],[-11,-2,13],[-11,-1,12],[-11,0,11],[-11,1,10],[-11,2,9],[-11,3,8],[-11,4,7],[-11,5,6],[-10,-4,14],[-10,-3,13],[-10,-2,12],[-10,-1,11],[-10,0,10],[-10,1,9],[-10,2,8],[-10,3,7],[-10,4,6],[-10,5,5],[-9,-5,14],[-9,-4,13],[-9,-3,12],[-9,-2,11],[-9,-1,10],[-9,0,9],[-9,1,8],[-9,2,7],[-9,3,6],[-9,4,5],[-8,-6,14],[-8,-5,13],[-8,-4,12],[-8,-3,11],[-8,-2,10],[-8,-1,9],[-8,0,8],[-8,1,7],[-8,2,6],[-8,3,5],[-8,4,4],[-7,-7,14],[-7,-6,13],[-7,-5,12],[-7,-4,11],[-7,-3,10],[-7,-2,9],[-7,-1,8],[-7,0,7],[-7,1,6],[-7,2,5],[-7,3,4],[-6,-6,12],[-6,-5,11],[-6,-4,10],[-6,-3,9],[-6,-2,8],[-6,-1,7],[-6,0,6],[-6,1,5],[-6,2,4],[-6,3,3],[-5,-5,10],[-5,-4,9],[-5,-3,8],[-5,-2,7],[-5,-1,6],[-5,0,5],[-5,1,4],[-5,2,3],[-4,-4,8],[-4,-3,7],[-4,-2,6],[-4,-1,5],[-4,0,4],[-4,1,3],[-4,2,2],[-3,-3,6],[-3,-2,5],[-3,-1,4],[-3,0,3],[-3,1,2],[-2,-2,4],[-2,-1,3],[-2,0,2],[-2,1,1],[-1,-1,2],[-1,0,1],[0,0,0]]
        )


if __name__ == '__main__':
    unittest.main()

