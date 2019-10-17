#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def is_safe(self, N, board, row, col):
        # Check for all the columns on the row that are to the
        # left of our target column.
        for i in range(col):
            if board[row][i] == 'Q':
                return False

        # Check all the upper diagonal to the left
        for i, j in zip(range(row, -1, -1), range(col, -1, -1)):
            if board[i][j] == 'Q':
                return False

        # Check all the lower diagonal to the left
        for i, j in zip(range(row, N), range(col, -1, -1)):
            if board[i][j] == 'Q':
                return False

        return True

    def solve(self, N, acc, board, col):
        if col >= N:
            acc.append([''.join(row) for row in board])
        else:
            for row in range(N):
                if self.is_safe(N, board, row, col):
                    board[row][col] = 'Q'
                    self.solve(N, acc, board, col + 1)
                    board[row][col] = '.'

    def nqueens(self, N):
        acc = []
        board = [['.' for _ in range(N)] for _ in range(N)]
        self.solve(N, acc, board, 0)

        return acc


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (2, []),
            (3, []),
            (4, [[".Q..", "...Q", "Q...", "..Q."], ["..Q.", "Q...", "...Q", ".Q.."]]),
        ]

    def test_nqueens(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.nqueens(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

