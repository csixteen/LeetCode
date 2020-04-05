#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/paint-house/

from itertools import islice
from typing import List, Tuple
import unittest


class Solution:
    def minCost(self, costs: List[List[int]]) -> int:
        """ Solution using memoization. """

        def paint_cost(n, color):
            if (n, color) in self.cache:
                return self.cache[(n, color)]

            total_cost = costs[n][color]

            if n == len(costs) - 1:
                pass
            elif color == 0:  # Red
                total_cost += min(paint_cost(n+1, 1), paint_cost(n+1, 2))
            elif color == 1:
                total_cost += min(paint_cost(n+1, 0), paint_cost(n+1, 2))
            else:
                total_cost += min(paint_cost(n+1, 0), paint_cost(n+1, 1))

            self.cache[(n, color)] = total_cost

            return total_cost

        self.cache = {}
        if not costs:
            return 0

        return min(paint_cost(0, 0), paint_cost(0, 1), paint_cost(0, 2))

    def minCost2(self, costs: List[List[int]]) -> int:
        """ Solution using dynamic programming. """
        if not costs:
            return 0

        for i in range(len(costs) - 2, -1, -1):
            costs[i][0] += min(costs[i+1][1], costs[i+1][2])
            costs[i][1] += min(costs[i+1][0], costs[i+1][2])
            costs[i][2] += min(costs[i+1][0], costs[i+1][1])

        return min(costs[0])

class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {
                    "costs": [
                        [17, 2, 17],
                        [16, 16, 5],
                        [14, 3, 19]
                    ]
                },
                10,
            ),
            (
                {
                    "costs": [
                        [5, 8, 6],
                        [19, 14, 13],
                        [7, 5, 12],
                        [14, 15, 17],
                        [3, 20, 10]
                    ],
                },
                43,
            ),
        ]

    def test_minCost(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.minCost(**_input))
            self.assertEqual(expected, s.minCost2(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
