#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/last-stone-weight/

from typing import List
import heapq
import unittest


class Solution:
    def lastStoneWeight(self, stones: List[int]) -> int:
        heap = list(map(int.__neg__, stones))
        heapq.heapify(heap)

        while heap:
            y = abs(heapq.heappop(heap))

            if not heap:
                return y

            x = abs(heapq.heappop(heap))

            if x != y:
                heapq.heappush(heap, -(y - x))

        return heap[0] if heap else 0


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {
                    "stones": [2,7,4,1,8,1],
                },
                1,
            ),
        ]

    def test_lastStoneWeight(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.lastStoneWeight(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
