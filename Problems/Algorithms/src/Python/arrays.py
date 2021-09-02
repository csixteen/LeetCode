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

    def maximumProduct(self, nums: List[int]) -> int:
        # The idea behind this algorithm is to compare the products
        # max1*max2*max3 with min1*min2*max1 , where max1 is the highest
        # number, max2 and max3 are the second and third highest numbers,
        # respectively, and min1 and min2 are the first and second smallest
        # numbers. The logic behind this is that when we multiply two negative
        # numbers, we get a positive number as result. So the product between
        # -10 and -11 is actually higher than, let's say, 2 and 3, despite the
        # fact that 2 and 3 are greater than -10 and -11.
        min1 = min2 = float('inf')
        max1 = max2 = max3 = float('-inf')

        for n in nums:
            if n < min1:
                min2 = min1
                min1 = n
            elif n < min2:
                min2 = n

            if n > max1:
                max3 = max2
                max2 = max1
                max1 = n
            elif n > max2:
                max3 = max2
                max2 = n
            elif n > max3:
                max3 = n

        return max(min1*min2*max1, max1*max2*max3)

    def fairCandySwap(self, aliceSizes: List[int], bobSizes: List[int]) -> List[int]:
        """
        https://leetcode.com/problems/fair-candy-swap/

        The total amount of candies that Alice has is S_a. Similarly, the total
        amount of candies that Bob has is S_b. In order for the candy swap to be
        fair, then Alice gives a box `x` to Bob and in exchange receives a box `y`
        from Bob, such that S_a - x + y = S_b + x - y. Finding a pair of boxes [x, y]
        is a matter of solving the equation ((S_a - S_b) / 2) + y = x.
        """
        sum_a = sum(aliceSizes)
        sum_b = sum(bobSizes)
        set_a = set(aliceSizes)

        for y in bobSizes:
            x = (sum_a - sum_b) / 2 + y
            if x in set_a:
                return [x, y]
