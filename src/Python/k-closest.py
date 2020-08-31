#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/k-closest-points-to-origin/

from typing import List
import heapq
import unittest


class Solution:
    def kClosest2(self, points: List[List[int]], K: int) -> List[List[int]]:
        """ Using Min Heap """
        def distance(p: List[int]) -> float:
            return (p[0]**2 + p[1]**2)**(1/2)

        distances = [(point, distance(point)) for point in points]
        heapq.heapify(distances)

        return [item[0] for item in heapq.nsmallest(K, distances, key=lambda x: x[1])]

    def kClosest(self, points: List[List[int]], K: int) -> List[List[int]]:
        """ Using built-in sort """
        return sorted(points, key=lambda p: p[0]**2 + p[1]**2)[:K]


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.s = Solution()

    def test_example1(self):
        self.assertEqual(
            [[-2, 2]],
            self.s.kClosest([[1, 3],[-2, 2]], 1),
        )

    def test_example2(self):
        self.assertEqual(
            [[3, 3],[-2, 4]],
            self.s.kClosest([[3, 3],[5, -1],[-2, 4]], 2),
        )


if __name__ == "__main__":
    unittest.main(verbosity=2)
