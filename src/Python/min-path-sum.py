#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/minimum-path-sum/

from typing import List
import unittest


class Solution:
    def minPathSum(self, grid: List[List[int]]) -> int:
        """
        Brute-force with memoization. The submission is accepted.
        """
        def cost(row: int, col: int):
            if (row, col) in self.cache:
                return self.cache[(row, col)]

            total_cost = grid[row][col]
            if row == len(grid) - 1 and col == len(grid[0]) - 1:
                pass
            else:
                down = cost(row + 1, col) if row < len(grid) - 1 else float("+inf")
                right = cost(row, col + 1) if col < len(grid[0]) - 1 else float("+inf")
                total_cost += min(down, right)

            self.cache[(row, col)] = total_cost

            return total_cost

        self.cache = {}
        if not grid:
            return 0

        return cost(0, 0)

    def minPathSum2(self, grid: List[List[int]]) -> int:
        """
        Dynamic programming.
        """
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


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"grid": [
                    [1, 3, 1],
                    [1, 5, 1],
                    [4, 2, 1]
                ]},
                7,
            ),
        ]

    def test_minPathSum(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.minPathSum(**_input))
            self.assertEqual(expected, s.minPathSum2(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
