#!/usr/bin/env python3
# coding: utf-8

from __future__ import annotations
from typing import List


class Solution:
    def sortedSquares(self, nums: List[int]) -> List[int]:
        n = len(nums)
        left, right, res = 0, n-1, [0]*n

        for i in range(n):
            if abs(nums[left]) > abs(nums[right]):
                square = nums[left]
                left += 1
            else:
                square = nums[right]
                right -= 1

            res[n-i-1] = square*square

        return res

    def findDisappearedNumbers(self, nums: List[int]) -> List[int]:
        for i in nums:
            j = abs(i)
            if nums[j-1] > 0:
                nums[j-1] *= -1

        print(nums)

        res = []
        for i in range(len(nums)):
            if nums[i] > 0:
                res.append(i+1)

        return res
