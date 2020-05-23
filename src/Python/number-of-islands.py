#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/number-of-islands/

from typing import List
import unittest


class Solution:
    def dfs(self, grid: List[List[str]], i: int, j: int):
        if i < 0 or j < 0 or i == len(grid) or j == len(grid[0]) or grid[i][j] == "0":
            return

        grid[i][j] = "0"

        self.dfs(grid, i - 1, j)
        self.dfs(grid, i + 1, j)
        self.dfs(grid, i, j - 1)
        self.dfs(grid, i, j + 1)

    def numIslands(self, grid: List[List[str]]) -> int:
        islands = 0

        i_len = len(grid)
        if i_len == 0:
            return islands

        j_len = len(grid[0])

        for i in range(i_len):
            for j in range(j_len):
                if grid[i][j] == "1":
                    islands += 1

                    self.dfs(grid, i, j)

        return islands


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                [],
                0,
            ),
            (
                [["0", "0"], ["0", "0"]],
                0,
            ),
            (
                [["1", "1"], ["1", "1"]],
                1,
            ),
            (
                [["1", "0"], ["0", "1"]],
                2,
            ),
            (
                [
                    ["1", "1", "1"],
                    ["0", "1", "0"],
                    ["1", "1", "1"],
                ],
                1,
            ),
            (
                [
                    ["1", "1", "1", "1", "0"],
                    ["1", "1", "0", "1", "0"],
                    ["1", "1", "0", "0", "0"],
                    ["0", "0", "0", "0", "0"],
                ],
                1,
            ),
            (
                [
                    ["1", "1", "0", "0", "0"],
                    ["1", "1", "0", "0", "0"],
                    ["0", "0", "1", "0", "0"],
                    ["0", "0", "0", "1", "1"],
                ],
                3,
            ),
        ]

    def test_numIslands(self):
        s = Solution()
        for _input, expected in self.input_expected:
            self.assertEqual(s.numIslands(_input), expected)


if __name__ == "__main__":
    unittest.main(verbosity=2)
