#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/flood-fill/

from typing import List
import unittest


class Solution:
    def paint(self, img: List[List[int]], i: int, j: int, ic: int, newColor: int, v: set):
        if i < 0 or j < 0 or i >= len(img) or j >= len(img[0]) or (i, j) in v or img[i][j] != ic:
            return

        img[i][j] = newColor
        v.add((i, j))

        self.paint(img, i-1, j, ic, newColor, v)
        self.paint(img, i+1, j, ic, newColor, v)
        self.paint(img, i, j-1, ic, newColor, v)
        self.paint(img, i, j+1, ic, newColor, v)

    def floodFill(
        self, image: List[List[int]], sr: int, sc: int, newColor: int
    ) -> List[List[int]]:
        visited = set()
        self.paint(image, sr, sc, image[sr][sc], newColor, visited)

        return image


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {
                    "image": [
                        [1,1,1],
                        [1,1,0],
                        [1,0,1],
                    ],
                    "sr": 1,
                    "sc": 1,
                    "newColor": 2,
                },
                [
                    [2,2,2],
                    [2,2,0],
                    [2,0,1],
                ],
            ),
            (
                {
                    "image": [
                        [0,0,0],
                        [0,1,1],
                    ],
                    "sr": 1,
                    "sc": 1,
                    "newColor": 1,
                },
                [
                    [0,0,0],
                    [0,1,1],
                ]
            ),
        ]
        
    def test_floodFill(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.floodFill(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
