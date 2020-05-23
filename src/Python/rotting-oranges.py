#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/rotting-oranges/

from collections import deque
from typing import Iterator, List, Tuple
import unittest


class Solution:
    def neighbors(self, grid, row, col) -> Iterator[Tuple]:
        n_rows = len(grid)
        n_cols = len(grid[0])

        for i, j in ((row - 1, col), (row + 1, col), (row, col - 1), (row, col + 1)):
            if 0 <= i < n_rows and 0 <= j < n_cols:
                yield i, j

    def orangesRotting(self, grid: List[List[int]]) -> int:
        # We start by collecting all the rotten oranges.
        # Each will be the root of a tree, so each has depth
        # 0 (zero).
        rotten = deque()
        for row in range(len(grid)):
            for col in range(len(grid[0])):
                if grid[row][col] == 2:
                    rotten.append((row, col, 0))

        minutes = 0

        while len(rotten) > 0:
            row, col, minutes = rotten.popleft()

            for i, j in self.neighbors(grid, row, col):
                if grid[i][j] == 1:
                    grid[i][j] = 2
                    rotten.append((i, j, minutes + 1))

        if any(grid[i][j] == 1 for i in range(len(grid)) for j in range(len(grid[0]))):
            return -1

        return minutes


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                [
                    [2, 1, 1],
                    [1, 1, 0],
                    [0, 1, 1]
                ],
                4,
            ),
            (
                [
                    [2, 1, 1],
                    [0, 1, 1],
                    [1, 0, 1]
                ],
                -1,
            ),
            (
                [[0,2]], 0,
            ),
            (
                [[1,2,1,1,2,1,1]],
                2,
            ),
        ]

    def test_orangesRotting(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.orangesRotting(_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
