#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def rotateMatrix(self, M):
        size = len(M)
        for row in range(size // 2):
            for col in range(row, size - row - 1):
                x, y = row, col
                tmp = M[x][y]
                for _ in range(5):
                    tmp2 = M[x][y]
                    M[x][y] = tmp
                    x, y  = y, size - x - 1
                    tmp = tmp2

        return M


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (
                [
                    [1, 2],
                    [3, 4]
                ],
                [
                    [3, 1],
                    [4, 2]
                ]
            ),
            (
                [
                    [1, 2, 3],
                    [4, 5, 6],
                    [7, 8, 9]
                ],
                [
                    [7, 4, 1],
                    [8, 5, 2],
                    [9, 6, 3]
                ]
            ),
            (
                [
                    [1, 2, 3, 4],
                    [5, 6, 7, 8],
                    [9, 10, 11, 12],
                    [13, 14, 15, 16]
                ],
                [
                    [13, 9, 5, 1],
                    [14, 10, 6, 2],
                    [15, 11, 7, 3],
                    [16, 12, 8, 4]
                ]
            )
        ]

    def test_rotateMatrix(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.rotateMatrix(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

