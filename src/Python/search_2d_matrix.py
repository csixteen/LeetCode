#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/search-a-2d-matrix
class Solution(object):
    def search_matrix(self, matrix, target):
        """
        :type matrix: list[list[int]]
        :type target: int
        :rtype: bool
        """
        num_rows = len(matrix)
        l, r = 0, num_rows-1
        i = l + (r - l) // 2
        c = len(bin(num_rows)[2:])
        row = matrix[i]
        while c > 0 and not (row[0] <= target <= row[len(row)-1]):
            if row[0] < target or row[len(row)-1] < target:
                l = i + 1
            else:
                r = i - 1
            i = l + (r - l) // 2
            row = matrix[i]
            c -= 1
        else:
            return row.count(target) > 0


import unittest

class TestSolution(unittest.TestCase):
    def test_search_matrix(self):
        matrix1 = [
            [1, 3, 5, 7],
            [10, 11, 16, 20],
            [23, 30, 34, 50]
        ]
        matrix2 = [[1], [3]]

        self.assertTrue(Solution().search_matrix(matrix1, 3))
        self.assertFalse(Solution().search_matrix(matrix1, 4))
        self.assertFalse(Solution().search_matrix(matrix1, 100))
        self.assertFalse(Solution().search_matrix(matrix2, 2))
        self.assertTrue(Solution().search_matrix(matrix2, 1))
        self.assertTrue(Solution().search_matrix(matrix2, 3))


if __name__ == "__main__":
    unittest.main()

