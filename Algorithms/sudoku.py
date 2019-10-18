#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    """
    Working brute-force solution, exhaustive search.
    Takes ages to finish...no good!
    """
    def solve2(self, board):
        def is_safe(row, col, number):
            # Check horizontal on the left
            for i in range(col):
                if board[row][i] == str(number):
                    return False

            # Check vertical above:
            for i in range(row):
                if board[i][col] == str(number):
                    return False

            # Check square
            square_top = (row // 3) * 3
            square_left = (col // 3) * 3
            for i in range(square_top, square_top + 3):
                for j in range(square_left, square_left + 3):
                    if board[i][j] == str(number):
                        return False

            return True

        def sudoku(row, col):
            if row == 9:
                return True
            elif col == 9:
                return sudoku(row + 1, 0)

            if board[row][col] == '.':
                for number in range(1, 10):
                    if is_safe(row, col, number):
                        board[row][col] = str(number)
                        if sudoku(row, col + 1):
                            return True
                        board[row][col] = '.'
                return False
            else:
                return sudoku(row, col + 1)

        sudoku(0, 0)
        return board

    """
    Much better solution. Instead of trying all combinations, it
    simply tries to place already valid numbers on each empty cell.
    """
    def solve(self, board):
        def values(row, col):
            all_numbers = {'1', '2', '3', '4', '5', '6', '7', '8', '9'}
            square_top = (row // 3) * 3
            square_left = (col // 3) * 3
            for i in range(9):
                all_numbers.discard(board[col][i])
                all_numbers.discard(board[i][row])
                all_numbers.discard(board[square_left + i // 3][square_top + i % 3])

            return all_numbers

        def sudoku(pairs):
            if len(pairs) == 0:
                return True

            row, col = pairs[0]
            for value in values(row, col):
                board[col][row] = value
                if sudoku(pairs[1:]):
                    return True

            board[col][row] = '.'
            return False

        pairs = [(row, col) for col in range(9) for row in range(9) if board[col][row] == '.']
        sudoku(pairs)
        return board


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (
                [
                    ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                    ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                    ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                    ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                    ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                    ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                    ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                    ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                    ['.', '.', '.', '.', '8', '.', '.', '7', '9']
                ], 
                []
            )
        ]

    def test_solve(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.solve(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

