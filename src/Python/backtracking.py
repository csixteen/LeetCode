#!/usr/bin/env python3
# coding: utf-8

from typing import List
import itertools


class Solution:
    def letterCombinations(self, digits: List[str]) -> List[str]:
        """ https://leetcode.com/problems/letter-combinations-of-a-phone-number/ """
        if not len(digits):
            return []

        keyboard = {
            '2': ['a', 'b', 'c'],
            '3': ['d', 'e', 'f'],
            '4': ['g', 'h', 'i'],
            '5': ['j', 'k', 'l'],
            '6': ['m', 'n', 'o'],
            '7': ['p', 'q', 'r', 's'],
            '8': ['t', 'u', 'v'],
            '9': ['w', 'x', 'y', 'z'],
        }

        tuples = list(itertools.product(*[keyboard[d] for d in digits]))

        return [''.join(x) for x in tuples]

    def minPathSum(self, grid: List[List[int]]) -> int:
        """ https://leetcode.com/problems/minimum-path-sum/ """
        dp = [[float("+inf")] * len(grid[0]) for _ in range(len(grid))]

        for row in reversed(range(len(grid))):
            for col in reversed(range(len(grid[0]))):
                dp[row][col] = grid[row][col]

                if row < len(grid) - 1 or col < len(grid[0]) - 1:
                    dp[row][col] += min(
                        dp[row+1][col] if row < len(grid) - 1 else float("inf"),
                        dp[row][col+1] if col < len(grid[0]) - 1 else float("inf"),
                    )

        return dp[0][0]
